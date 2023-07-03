require_relative './Expr.rb'
require_relative './Stmt.rb'
require_relative './Token.rb'
require_relative './TokenType.rb'
require_relative './error.rb'

class ParseError < StandardError
end

class Parser
  def initialize(tokens, repl_mode=false)
    @tokens = tokens
    @current = 0
    @repl_mode = true
  end

  # program -> declaration* EOF
  # repl -> (declaration* | expression) EOF
  def parse()
    statements = []
    if !isAtEnd()
      statements << declaration(repl_expression=@repl_mode)
    end
    while !isAtEnd()
      statements << declaration()
    end
    statements
  end
  
  def declaration(repl_expression=false)
    begin
      return function("function") if match(TokenType::FUN)
      return classDeclaration() if match(TokenType::CLASS)
      return varDeclaration() if match(TokenType::VAR)
      statement(repl_expression)
    rescue ParseError => e
      synchronize()
      nil
    end
  end

  def function(kind)
    name = consume(TokenType::IDENTIFIER, "Expect #{kind} name.")
    consume(TokenType::LEFT_PAREN, "Expect '(' after #{kind} name.")
    parameters = []
    if !check(TokenType::RIGHT_PAREN)
      begin
        if parameters.length >= 255
          error(peek(), "Can't have more than 255 parameters.")
        end
        parameters << consume(TokenType::IDENTIFIER, "Expect parameter name.")
      end while match(TokenType::COMMA)
    end
    consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.")
    consume(TokenType::LEFT_BRACE, "Expect '{' before #{kind} body.")
    body = block()
    
    FunctionStmt.new(name, parameters, body)
  end

  def classDeclaration()
    name = consume(TokenType::IDENTIFIER, "Expect class name.")

    superclass = nil
    if match(TokenType::LESS)
      consume(TokenType::IDENTIFIER, "Expect superclass name.")
      superclass = Variable.new(previous())
    end
    
    consume(TokenType::LEFT_BRACE, "Expect '{' before class body.")
    methods = []
    while !check(TokenType::RIGHT_BRACE) && !isAtEnd()
      methods << function("method")
    end
    consume(TokenType::RIGHT_BRACE, "Expect '}' after class body.")
    ClassStmt.new(name, superclass, methods)
  end

  def varDeclaration()
    name = consume(TokenType::IDENTIFIER, "Expect variable name.")
    initializer = nil
    initializer = expression() if match(TokenType::EQUAL)
    consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.")
    VarStmt.new(name, initializer)
  end

  def statement(repl_expression=false)
    return forStatement() if match(TokenType::FOR)
    return ifStatement() if match(TokenType::IF)
    return whileStatement() if match(TokenType::WHILE)
    return returnStatement() if match(TokenType::RETURN)
    return printStatement() if match(TokenType::PRINT)
    return BlockStmt.new(block()) if match(TokenType::LEFT_BRACE)
    expressionStatement(repl_expression)
  end

  def returnStatement()
    keyword = previous()
    value = nil
    value = expression() if !check(TokenType::SEMICOLON)
    consume(TokenType::SEMICOLON, "Expect ';' after return value.")
    ReturnStmt.new(keyword, value)
  end

  def forStatement()
    consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")
    initializer = nil
    if match(TokenType::SEMICOLON)
      initializer = expressionStatement()
    elsif match(TokenType::VAR)
      initializer = varDeclaration()
    else
      initializer = expressionStatement()
    end
    condition = nil
    if !check(TokenType::SEMICOLON)
      condition = expression()
    end
    consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")
    increment = nil
    increment = expression() if !check(TokenType::RIGHT_PAREN)
    consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")
    body = statement()
    if not increment.nil?
      body = BlockStmt.new([body, ExprStmt.new(increment)])
    end

    if condition.nil?
      condition = Literal.new(true)
    end
    body = WhileStmt.new(condition, body)
    if not initializer.nil?
      body = BlockStmt.new([initializer, body])
    end
    body
  end
    

  def whileStatement()
    consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")
    condition = expression()
    consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.")
    body = statement()
    WhileStmt.new(condition, body)
  end

  def ifStatement()
    consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")
    condition = expression()
    consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")
    then_branch = statement()
    else_branch = statement() if match(TokenType::ELSE)
    IfStmt.new(condition, then_branch, else_branch)
  end

  def block()
    statements = []
    while !check(TokenType::RIGHT_BRACE) && !isAtEnd()
      statements << declaration()
    end
    consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")
    statements
  end

  def printStatement()
    value = expression()
    consume(TokenType::SEMICOLON, "Expect ';' after value.")
    PrintStmt.new(value)
  end

  def expressionStatement(repl_expression=false)
    expr = expression()

    # If we're in REPL mode, it's ok to have
    # an expression without ma semicolon.
    return ExprStmt.new(expr) if repl_expression and isAtEnd()
      
    consume(TokenType::SEMICOLON, "Expect ';' after expression.")
    ExprStmt.new(expr)
  end

  def expression()
    assignment
  end

  def assignment
    expr = orExpr()
    if match(TokenType::EQUAL)
      equals = previous()
      value = assignment()
      if expr.is_a?(Variable)
        name = expr.name
        return Assign.new(name, value)
      elsif expr.is_a?(Get)
        get = expr
        return SetExpr.new(get.object, get.name, value)
      end
      error(equals, "Invalid assignment target.")
    end
    expr
  end

  def orExpr()
    expr = andExpr()
    while match(TokenType::OR)
      operator = previous()
      right = andExpr()
      expr = Logical.new(expr, operator, right)
    end
    expr
  end

  def andExpr()
    expr = equality()
    while match(TokenType::AND)
      operator = previous()
      right = equality()
      expr = Logical.new(expr, operator, right)
    end
    expr
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
    call()
  end

  def call()
    expr = primary()
    while true
      if match(TokenType::LEFT_PAREN)
        expr = finishCall(expr)
      elsif match(TokenType::DOT)
        name = consume(TokenType::IDENTIFIER, "Expect property name after '.'.")
        expr = Get.new(expr, name)
      else
        break
      end
    end
    expr
  end

  def finishCall(callee)
    arguments = []
    if !check(TokenType::RIGHT_PAREN)
      begin
        if arguments.length >= 255
          error(peek(), "Can't have more than 255 arguments.")
        end
        arguments << expression()
      end while match(TokenType::COMMA)
    end
    paren = consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.")
    Call.new(callee, paren, arguments)
  end

  def primary()
    return Literal.new(false) if match(TokenType::FALSE)
    return Literal.new(true) if match(TokenType::TRUE)
    return Literal.new(nil) if match(TokenType::NIL)
    return This.new(previous()) if match(TokenType::THIS)
    return Literal.new(previous().literal) if match(TokenType::NUMBER, TokenType::STRING)
    return Variable.new(previous()) if match(TokenType::IDENTIFIER)
    if match(TokenType::LEFT_PAREN)
      expr = expression()
      consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")
      return Grouping.new(expr)
    end
    if match(TokenType::SUPER)
      keyword = previous()
      consume(TokenType::DOT, "Expect '.' after 'super'.")
      method = consume(TokenType::IDENTIFIER, "Expect superclass method name.")
      return Super.new(keyword, method)
    end
    raise error(peek(), "Expect expression.")
  end

  def consume(type, message)
    return advance if check(type)
    error(peek(), message)
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
