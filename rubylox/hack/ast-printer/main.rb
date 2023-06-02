# ast-printer entrypoint

require_relative '../../Expr.rb'
require_relative '../../TokenType.rb'
require_relative '../../Token.rb'
require_relative '../../AstPrinter.rb'

def main
  expr = Binary.new(
    Unary.new(
      Token.new(TokenType::MINUS, "-", nil, 1),
      Literal.new(123)),
    Token.new(TokenType::STAR, "*", nil, 1),
    Grouping.new(
      Literal.new(45.67)))
  puts AstPrinter.new().print(expr)
end
