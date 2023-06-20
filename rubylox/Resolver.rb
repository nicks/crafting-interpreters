require_relative './Expr.rb'
require_relative './Stmt.rb'

module FunctionType
  NONE = :none
  FUNCTION = :function
end

class Resolver
  include ExprVisitor
  include StmtVisitor

  def initialize(interpreter)
    @interpreter = interpreter
    @scopes = []
    @current_function = FunctionType::NONE
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
    resolve(stmt.value) if not stmt.value.nil?
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
  end

  def define(name)
    return if @scopes.empty?
    @scopes.last[name.lexeme] = true
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
