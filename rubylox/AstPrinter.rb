# ast-printer entrypoint
# pretty-print an ast

require_relative './Expr.rb'
require_relative './Token.rb'
require_relative './TokenType.rb'
require 'stringio'

class AstPrinter
  include ExprVisitor
  
  def print(expr)
    expr.accept(self)
  end

  def visitBinary(expr)
    parenthesize(expr.operator.lexeme, expr.left, expr.right)
  end

  def visitGrouping(expr)
    parenthesize("group", expr.expression)
  end

  def visitLiteral(expr)
    if expr.value.nil?
      return "nil"
    end
    expr.value.to_s
  end

  def visitUnary(expr)
    parenthesize(expr.operator.lexeme, expr.right)
  end

  def parenthesize(name, *exprs)
    s = StringIO.new
    s << "(" << name
    exprs.each do |expr|
      s << " " << expr.accept(self)
    end
    s << ")"
    s.string
  end
end
