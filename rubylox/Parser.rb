require_relative './Expr.rb'
require_relative './Token.rb'
require_relative './TokenType.rb'
require_relative './error.rb'

class ParseError < StandardError
end

class Parser
  def initialize(tokens)
    @tokens = tokens
    @current = 0
  end

  def parse()
    begin
      expression()
    rescue ParseError => e
      return nil
    end
  end

  def expression()
    equality
  end

  def equality()
    expr = comparison()

    while match(TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL)
      operator = previous()
      right = comparison()
      expr = Binary.new(expr, operator, right)
    end

    return expr
  end

  def comparison()
    expr = term()
    while match(TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL)
      operator = previous()
      right = term()
      expr = Binary.new(expr, operator, right)
    end
    expr
  end

  def term()
    expr = factor()
    while match(TokenType::MINUS, TokenType::PLUS)
      operator = previous()
      right = factor()
      expr = Binary.new(expr, operator, right)
    end
    expr
  end

  def factor()
    expr = unary()
    while match(TokenType::SLASH, TokenType::STAR)
      operator = previous()
      right = unary()
      expr = Binary.new(expr, operator, right)
    end
    expr
  end

  def unary()
    if match(TokenType::BANG, TokenType::MINUS)
      operator = previous()
      right = unary()
      return Unary.new(operator, right)
    end
    primary()
  end

  def primary()
    return Literal.new(false) if match(TokenType::FALSE)
    return Literal.new(true) if match(TokenType::TRUE)
    return Literal.new(nil) if match(TokenType::NIL)
    return Literal.new(previous().literal) if match(TokenType::NUMBER, TokenType::STRING)
    if match(TokenType::LEFT_PAREN)
      expr = expression()
      consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")
      return Grouping.new(expr)
    end
    raise error(peek(), "Expect expression.")
  end

  def consume(type, message)
    advance if check(type)
    throw error(peek(), message)
  end

  def error(token, message)
    Err.parse_error(token, message)
    raise ParseError.new()
  end
  
  def match(*types)
    types.each do |type|
      if check(type)
        advance()
        return true
      end
    end

    return false
  end

  def check(type)
    return false if isAtEnd()
    peek().type == type
  end

  def advance()
    @current += 1 unless isAtEnd()
    previous()
  end

  def isAtEnd()
    peek().type == TokenType::EOF
  end

  def peek()
    @tokens[@current]
  end

  def previous()
    @tokens[@current - 1]
  end

  def synchronize()
    advance()
    while !isAtEnd()
      return if previous().type == TokenType::SEMICOLON

      case peek().type
      when TokenType::CLASS
      when TokenType::FUN
      when TokenType::VAR
      when TokenType::FOR
      when TokenType::IF
      when TokenType::WHILE
      when TokenType::PRINT
      when TokenType::RETURN
        return
      end
      advance()
    end
  end
  
end
