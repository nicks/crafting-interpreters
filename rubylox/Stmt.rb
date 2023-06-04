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

  def visitPrintStmt(stmt)
    raise NotImplementedError
  end

  def visitVarStmt(stmt)
    raise NotImplementedError
  end

  def visitBlockStmt(stmt)
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

class PrintStmt < Stmt
  attr_reader :expression
  def initialize(expression)
    @expression = expression
  end

  def accept(visitor)
    visitor.visitPrintStmt(self)
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

