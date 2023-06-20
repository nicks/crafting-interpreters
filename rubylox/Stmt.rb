# Path: ./Stmt.rb
# Desc: Auto-generated AST for Stmt

class Stmt
  def accept(visitor)
    raise NotImplementedError
  end
end

module StmtVisitor
  def visitExprStmt(stmt)
    raise NotImplementedError
  end

  def visitIfStmt(stmt)
    raise NotImplementedError
  end

  def visitPrintStmt(stmt)
    raise NotImplementedError
  end

  def visitReturnStmt(stmt)
    raise NotImplementedError
  end

  def visitVarStmt(stmt)
    raise NotImplementedError
  end

  def visitBlockStmt(stmt)
    raise NotImplementedError
  end

  def visitWhileStmt(stmt)
    raise NotImplementedError
  end

  def visitFunctionStmt(stmt)
    raise NotImplementedError
  end

  def visitClassStmt(stmt)
    raise NotImplementedError
  end

end


class ExprStmt < Stmt
  attr_reader :expression
  def initialize(expression)
    @expression = expression
  end

  def accept(visitor)
    visitor.visitExprStmt(self)
  end

end

class IfStmt < Stmt
  attr_reader :condition, :then_branch, :else_branch
  def initialize(condition, then_branch, else_branch)
    @condition = condition
    @then_branch = then_branch
    @else_branch = else_branch
  end

  def accept(visitor)
    visitor.visitIfStmt(self)
  end

end

class PrintStmt < Stmt
  attr_reader :expression
  def initialize(expression)
    @expression = expression
  end

  def accept(visitor)
    visitor.visitPrintStmt(self)
  end

end

class ReturnStmt < Stmt
  attr_reader :keyword, :value
  def initialize(keyword, value)
    @keyword = keyword
    @value = value
  end

  def accept(visitor)
    visitor.visitReturnStmt(self)
  end

end

class VarStmt < Stmt
  attr_reader :name, :initializer
  def initialize(name, initializer)
    @name = name
    @initializer = initializer
  end

  def accept(visitor)
    visitor.visitVarStmt(self)
  end

end

class BlockStmt < Stmt
  attr_reader :statements
  def initialize(statements)
    @statements = statements
  end

  def accept(visitor)
    visitor.visitBlockStmt(self)
  end

end

class WhileStmt < Stmt
  attr_reader :condition, :body
  def initialize(condition, body)
    @condition = condition
    @body = body
  end

  def accept(visitor)
    visitor.visitWhileStmt(self)
  end

end

class FunctionStmt < Stmt
  attr_reader :name, :params, :body
  def initialize(name, params, body)
    @name = name
    @params = params
    @body = body
  end

  def accept(visitor)
    visitor.visitFunctionStmt(self)
  end

end

class ClassStmt < Stmt
  attr_reader :name, :methods
  def initialize(name, methods)
    @name = name
    @methods = methods
  end

  def accept(visitor)
    visitor.visitClassStmt(self)
  end

end

