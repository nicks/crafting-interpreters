# Path: ./Expr.rb
# Desc: Auto-generated AST for Expr

class Expr
  def accept(visitor)
    raise NotImplementedError
  end
end

module Visitor
  def visitBinary(expr)
    raise NotImplementedError
  end

  def visitGrouping(expr)
    raise NotImplementedError
  end

  def visitLiteral(expr)
    raise NotImplementedError
  end

  def visitUnary(expr)
    raise NotImplementedError
  end

end


class Binary < Expr
  attr_reader :left, :operator, :right
  def initialize(left, operator, right)
    @left = left
    @operator = operator
    @right = right
  end

  def accept(visitor)
    visitor.visitBinary(self)
  end

end

class Grouping < Expr
  attr_reader :expression
  def initialize(expression)
    @expression = expression
  end

  def accept(visitor)
    visitor.visitGrouping(self)
  end

end

class Literal < Expr
  attr_reader :value
  def initialize(value)
    @value = value
  end

  def accept(visitor)
    visitor.visitLiteral(self)
  end

end

class Unary < Expr
  attr_reader :operator, :right
  def initialize(operator, right)
    @operator = operator
    @right = right
  end

  def accept(visitor)
    visitor.visitUnary(self)
  end

end
