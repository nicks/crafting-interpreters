require_relative './Expr.rb'
require_relative './Stmt.rb'

module FunctionType
  NONE = :none
  FUNCTION = :function
  METHOD = :method
  INITIALIZER = :initializer
end

module ClassType
  NONE = :none
  CLASS = :class
  SUBCLASS = :subclass
end

class Resolver
  include ExprVisitor
  include StmtVisitor

  def initialize(interpreter)
    @interpreter = interpreter
    @scopes = []
    @current_function = FunctionType::NONE
    @current_class = ClassType::NONE
  end

  def visitBlockStmt(stmt)
    beginScope()
    resolveStmts(stmt.statements)
    endScope()
    nil
  end

  def visitVarStmt(stmt)
    declare(stmt.name)
    resolve(stmt.initializer) if not stmt.initializer.nil?
    define(stmt.name)
    nil
  end

  def visitAssign(expr)
    resolve(expr.value)
    resolveLocal(expr, expr.name)
  end

  def visitFunctionStmt(stmt)
    declare(stmt.name)
    define(stmt.name)
    resolveFunction(stmt, FunctionType::FUNCTION)
  end

  def visitClassStmt(stmt)
    enclosing_class = @current_class
    @current_class = ClassType::CLASS
   
    declare(stmt.name)
    define(stmt.name)

    beginScope()
    @scopes.last["this"] = true

    stmt.methods.each do |method|
      function_type = FunctionType::METHOD
      if method.name.lexeme == "init"
        function_type = FunctionType::INITIALIZER
      end
      resolveFunction(method, function_type)
    end

    endScope()

    @current_class = enclosing_class
  end

  def resolveFunction(function, type)
    enclosing_function = @current_function
    @current_function = type
    
    beginScope()
    function.params.each do |param|
      declare(param)
      define(param)
    end
    resolveStmts(function.body)
    endScope()

    @current_function = enclosing_function
  end

  def visitExprStmt(stmt)
    resolve(stmt.expression)
  end

  def visitIfStmt(stmt)
    resolve(stmt.condition)
    resolve(stmt.then_branch)
    resolve(stmt.else_branch) if not stmt.else_branch.nil?
  end

  def visitPrintStmt(stmt)
    resolve(stmt.expression)
  end

  def visitReturnStmt(stmt)
    if @current_function == FunctionType::NONE
      Err.error(stmt.keyword, "Can't return from top-level code.")
    end
    if not stmt.value.nil?
      if @current_function == FunctionType::INITIALIZER
        Err.error(stmt.keyword, "Can't return a value from an initializer.")
      end
      resolve(stmt.value)
    end
  end

  def visitWhileStmt(stmt)
    resolve(stmt.condition)
    resolve(stmt.body)
  end

  def visitBinary(expr)
    resolve(expr.left)
    resolve(expr.right)
  end

  def visitCall(expr)
    resolve(expr.callee)
    expr.arguments.each do |arg|
      resolve(arg)
    end
  end

  def visitGet(expr)
    resolve(expr.object)
  end

  def visitSetExpr(expr)
    resolve(expr.value)
    resolve(expr.object)
  end

  def visitThis(expr)
    if @current_class == ClassType::NONE
      Err.error(expr.keyword, "Can't use 'this' outside of a class.")
      return
    end
    resolveLocal(expr, expr.keyword)
  end

  def visitGrouping(expr)
    resolve(expr.expression)
  end

  def visitLiteral(expr)
    nil
  end

  def visitLogical(expr)
    resolve(expr.left)
    resolve(expr.right)
  end

  def visitUnary(expr)
    resolve(expr.right)
  end
    
  def visitVariable(expr)
    if !@scopes.empty? && @scopes.last[expr.name.lexeme] == false
      Err.error(expr.name, "Can't read local variable in its own initializer.")
    end
    resolveLocal(expr, expr.name)
    nil
  end

  def declare(name)
    return if @scopes.empty?
    if @scopes.last.key?(name.lexeme)
      Err.error(name, "Variable with this name already declared in this scope.")
    end
    @scopes.last[name.lexeme] = false
    nil
  end

  def define(name)
    return if @scopes.empty?
    @scopes.last[name.lexeme] = true
    nil
  end

  def resolveStmts(statements)
    statements.each do |statement|
      resolve(statement)
    end
  end

  def resolve(stmt)
    stmt.accept(self)
    nil
  end

  def resolveLocal(expr, name)
    for i in (@scopes.length - 1).downto(0)
      if @scopes[i].key?(name.lexeme)
        @interpreter.resolve(expr, @scopes.length - 1 - i)
        return
      end
    end
    nil
  end

  def beginScope()
    @scopes << {}
    nil
  end

  def endScope()
    @scopes.pop()
    nil
  end
end
