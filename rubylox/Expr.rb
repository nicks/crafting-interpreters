# Path: ./Expr.rb
# Desc: Auto-generated AST for Expr

class Expr
  def accept(visitor)
    raise NotImplementedError
  end
end

module ExprVisitor
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

  def visitVariable(expr)
    raise NotImplementedError
  end

  def visitAssign(expr)
    raise NotImplementedError
  end

  def visitLogical(expr)
    raise NotImplementedError
  end

  def visitCall(expr)
    raise NotImplementedError
  end

  def visitGet(expr)
    raise NotImplementedError
  end

  def visitSetExpr(expr)
    raise NotImplementedError
  end

  def visitSuper(expr)
    raise NotImplementedError
  end

  def visitThis(expr)
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

class Variable < Expr
  attr_reader :name
  def initialize(name)
    @name = name
  end

  def accept(visitor)
    visitor.visitVariable(self)
  end

end

class Assign < Expr
  attr_reader :name, :value
  def initialize(name, value)
    @name = name
    @value = value
  end

  def accept(visitor)
    visitor.visitAssign(self)
  end

end

class Logical < Expr
  attr_reader :left, :operator, :right
  def initialize(left, operator, right)
    @left = left
    @operator = operator
    @right = right
  end

  def accept(visitor)
    visitor.visitLogical(self)
  end

end

class Call < Expr
  attr_reader :callee, :paren, :arguments
  def initialize(callee, paren, arguments)
    @callee = callee
    @paren = paren
    @arguments = arguments
  end

  def accept(visitor)
    visitor.visitCall(self)
  end

end

class Get < Expr
  attr_reader :object, :name
  def initialize(object, name)
    @object = object
    @name = name
  end

  def accept(visitor)
    visitor.visitGet(self)
  end

end

class SetExpr < Expr
  attr_reader :object, :name, :value
  def initialize(object, name, value)
    @object = object
    @name = name
    @value = value
  end

  def accept(visitor)
    visitor.visitSetExpr(self)
  end

end

class Super < Expr
  attr_reader :keyword, :method
  def initialize(keyword, method)
    @keyword = keyword
    @method = method
  end

  def accept(visitor)
    visitor.visitSuper(self)
  end

end

class This < Expr
  attr_reader :keyword
  def initialize(keyword)
    @keyword = keyword
  end

  def accept(visitor)
    visitor.visitThis(self)
  end

end

