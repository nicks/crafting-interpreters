require_relative './Expr.rb'

class RuntimeError < StandardError
  attr_reader :token, :message
  
  def initialize(token, message)
    @token = token
    @message = message
  end

  def to_s
    "#{@message}"
  end
end

class Return < StandardError
  attr_reader :value
  def initialize(value)
    @value = value
  end
end

class Environment
  def initialize(enclosing=nil)
    @values = {}
    @enclosing = enclosing
  end

  def define(name, value)
    @values[name.lexeme] = value
  end
  
  def define_native(name, value)
    @values[name] = value
  end

  def get(name)
    return @values[name.lexeme] if @values.key?(name.lexeme)
    return @enclosing.get(name) if @enclosing
    raise RuntimeError.new(name, "Undefined variable '#{name.lexeme}'.")
  end

  def assign(name, value)
    if @values.key?(name.lexeme)
      @values[name.lexeme] = value
      return
    end
    return @enclosing.assign(name, value) if @enclosing
    raise RuntimeError.new(name, "Undefined variable '#{name.lexeme}'.")
  end
end

class LoxCallable
  def arity
    raise NotImplementedError
  end

  def call
    raise NotImplementedError
  end
end

class NativeCallable < LoxCallable
  attr_reader :arity
  
  def initialize(arity=0, &block)
    @arity = arity
    @block = block
  end
  
  def call(interpreter, arguments)
    @block.call(interpreter, arguments)
  end

  def to_s
    "<native fn>"
  end
end

class LoxFunction < LoxCallable
  def initialize(declaration, closure)
    @declaration = declaration
    @closure = closure
  end

  def call(interpreter, arguments)
    environment = Environment.new(@closure)
    @declaration.params.each_with_index do |param, i|
      environment.define(param, arguments[i])
    end
    begin
      interpreter.executeBlock(@declaration.body.statements, environment)
    rescue Return => returnValue
      return returnValue.value
    end
    nil
  end

  def arity
    @declaration.params.length
  end

  def to_s
    "<fn #{@declaration.name.lexeme}>"
  end
end

$clock = NativeCallable.new(0) { |interpreter, arguments| Time.now.to_f }

class Interpreter
  include ExprVisitor
  include StmtVisitor

  attr_reader :globals

  def initialize(repl_mode=false)
    @globals = Environment.new
    @globals.define_native("clock", $clock)
    
    @environment = @globals
    @repl_mode = repl_mode
  end
  
  def interpret(stmts)
    begin
      *list, last = stmts
      list.each do |stmt|
        execute(stmt)
      end
      if @repl_mode and last.is_a?(ExprStmt)
        puts stringify(evaluate(last.expression))
      elsif last
        execute(last)
      end        
    rescue RuntimeError => e
      Err.runtime_error(e)
    end
  end

  def stringify(object)
    return "nil" if object.nil?
    return object if object.is_a?(String)
    if object.is_a?(Numeric)
      text = object.to_s
      return text.to_i if text.end_with?(".0")
      return text
    end
    object.to_s
  end

  def visitVariable(expr)
    @environment.get(expr.name)
  end

  def visitVarStmt(stmt)
    value = nil
    value = evaluate(stmt.initializer) if stmt.initializer
    @environment.define(stmt.name, value)
    nil
  end

  def visitIfStmt(stmt)
    if isTruthy(evaluate(stmt.condition))
      execute(stmt.then_branch)
    elsif stmt.else_branch
      execute(stmt.else_branch)
    end
    nil
  end

  def visitFunctionStmt(stmt)
    function = LoxFunction.new(stmt, @environment)
    @environment.define(stmt.name, function)
    nil
  end

  def visitReturnStmt(stmt)
    value = nil
    value = evaluate(stmt.value) if not stmt.value.nil?
    raise Return.new(value)
  end
  
  def visitWhileStmt(stmt)
    while isTruthy(evaluate(stmt.condition))
      execute(stmt.body)
    end
    nil
  end

  def visitCall(expr)
    callee = evaluate(expr.callee)
    arguments = []
    expr.arguments.each do |arg|
      arguments << evaluate(arg)
    end
    if !callee.is_a?(LoxCallable)
      raise RuntimeError.new(expr.paren, "Can only call functions and classes.")
    end
    if arguments.length != callee.arity()
      raise RuntimeError.new(
              expr.paren,
              "Expected #{callee.arity()} arguments but got #{arguments.length}.")
    end
    callee.call(self, arguments)
  end
  
  def visitLogical(expr)
    left = evaluate(expr.left)
    if expr.operator.type == TokenType::OR
      return left if isTruthy(left)
    else
      return left unless isTruthy(left)
    end
    evaluate(expr.right)
  end

  def visitAssign(expr)
    value = evaluate(expr.value)
    @environment.assign(expr.name, value)
    value
  end

  def visitBlockStmt(stmt)
    executeBlock(stmt.statements, Environment.new(@environment))
    nil
  end
  
  def visitExprStmt(stmt)
    evaluate(stmt.expression)
    nil
  end

  def visitPrintStmt(stmt)
    value = evaluate(stmt.expression)
    puts stringify(value)
    nil
  end

  def visitLiteral(expr)
    expr.value
  end
  
  def visitGrouping(expr)
    evaluate(expr.expression)
  end

  def visitUnary(expr)
    right = evaluate(expr.right)
    case expr.operator.type
    when TokenType::MINUS
      checkNumberOperand(expr.operator, right)
      -right
    when TokenType::BANG then !isTruthy(right)
    else nil
    end
  end

  def isTruthy(object)
    return false if object.nil?
    return false if object.is_a?(FalseClass)
    true
  end

  def visitBinary(expr)
    op = expr.operator
    left = evaluate(expr.left)
    right = evaluate(expr.right)
    case op.type
    when TokenType::MINUS
      checkNumberOperands(op, left, right)
      left - right
    when TokenType::SLASH
      checkNumberOperands(op, left, right)
      left / right
    when TokenType::STAR
      checkNumberOperands(op, left, right)
      left * right
    when TokenType::PLUS
      if left.is_a?(Numeric) && right.is_a?(Numeric)
        left + right
      elsif left.is_a?(String) && right.is_a?(String)
        left + right
      else
        raise RuntimeError.new(op,
                               "Operands must be two numbers or two strings.")
      end
    when TokenType::GREATER 
      checkNumberOperands(op, left, right)
      left > right
    when TokenType::GREATER_EQUAL 
      checkNumberOperands(op, left, right)
      left >= right
    when TokenType::LESS 
      checkNumberOperands(op, left, right)
      left < right
    when TokenType::LESS_EQUAL 
      checkNumberOperands(op, left, right)
      left <= right
    when TokenType::BANG_EQUAL then !isEqual(left, right)
    when TokenType::EQUAL_EQUAL then isEqual(left, right)
    else nil
    end
  end

  def isEqual(a, b)
    return true if a.nil? && b.nil?
    return false if a.nil?
    a == b
  end

  def evaluate(expr)
    expr.accept(self)
  end

  def execute(stmt)
    stmt.accept(self)
    nil
  end

  def executeBlock(stmts, environment)
    previous = @environment
    begin
      @environment = environment
      stmts.each do |stmt|
        execute(stmt)
      end
    ensure
      @environment = previous
    end
  end

  def checkNumberOperands(operator, left, right)
    return if left.is_a?(Numeric) && right.is_a?(Numeric)
    raise RuntimeError.new(operator, "Operands must be numbers.")
  end

  def checkNumberOperand(operator, x)
    return if x.is_a?(Numeric)
    raise RuntimeError.new(operator, "Operand must be number.")
  end
end
