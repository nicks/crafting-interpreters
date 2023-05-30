require "./error.rb"
require "./Token.rb"
require "./TokenType.rb"

class Scanner
  def initialize(source)
    @source = source
    @tokens = []
    @start = 0
    @current = 0
    @line = 1
  end

  def scanTokens
    until isAtEnd
      # We are at the beginning of the next lexeme.
      @start = @current
      scanToken
    end
    @tokens.push(Token.new(TokenType::EOF, "", nil, @line))
    @tokens
  end

  def isAtEnd
    @current >= @source.length
  end

  def advance
    result = @source[@current]
    @current += 1
    result
  end

  def match(expected)
    return false if isAtEnd
    return false if @source[@current] != expected

    @current += 1
    true
  end

  def peek
    return "\0" if isAtEnd
    @source[@current]
  end

  def peekNext
    return "\0" if @current + 1 >= @source.length
  end

  def addToken(type, literal = nil)
    text = @source[@start...@current]
    @tokens.push(Token.new(type, text, literal, @line))
  end

  def scanToken
    c = advance
    case c
    when "(" then addToken(TokenType::LEFT_PAREN)
    when ")" then addToken(TokenType::RIGHT_PAREN)
    when "{" then addToken(TokenType::LEFT_BRACE)
    when "}" then addToken(TokenType::RIGHT_BRACE)
    when "," then addToken(TokenType::COMMA)
    when "." then addToken(TokenType::DOT)
    when "-" then addToken(TokenType::MINUS)
    when "+" then addToken(TokenType::PLUS)
    when ";" then addToken(TokenType::SEMICOLON)
    when "*" then addToken(TokenType::STAR)
    when "!" then addToken(match("=") ? TokenType::BANG_EQUAL : TokenType::BANG)
    when "=" then addToken(match("=") ? TokenType::EQUAL_EQUAL : TokenType::EQUAL)
    when "<" then addToken(match("=") ? TokenType::LESS_EQUAL : TokenType::LESS)
    when ">" then addToken(match("=") ? TokenType::GREATER_EQUAL : TokenType::GREATER)
    when "/" then
      if match("/")
        # A comment goes until the end of the line.
        while peek != "\n" && !isAtEnd
          advance
        end
      else
        addToken(TokenType::SLASH)
      end
    when " ", "\r", "\t"
      # Ignore whitespace.
    when "\n"
      @line += 1
    when '"'
      string
    else
      if isDigit(c)
        number
      elsif isAlpha(c)
        identifier
      else
        Err.error(@line, "Unexpected character.")
      end
    end
  end

  def string
    while peek != '"' && !isAtEnd
      @line += 1 if peek == "\n"
      advance
    end

    if isAtEnd
      Err.error(@line, "Unterminated string.")
      return
    end

    advance # closing "

    # trim quotes
    addToken(TokenType::STRING, @source[@start + 1, @current - @start - 2])
  end

  def number
    while isDigit(peek)
      advance
    end

    # Look for a fractional part.
    if peek == "." && isDigit(peekNext)
      # Consume the "."
      advance

      while isDigit(peek)
        advance
      end
    end

    addToken(TokenType::NUMBER, @source[@start, @current - @start].to_f)
  end

  def isDigit(c)
    c >= "0" && c <= "9"
  end

  def isAlpha(c)
    c >= "a" && c <= "z" ||
    c >= "A" && c <= "Z" ||
    c == "_"
  end

  def isAlphaNumeric(c)
    isAlpha(c) || isDigit(c)
  end 

  def identifier
    while isAlphaNumeric(peek)
      advance
    end

    text = @source[@start, @current - @start]
    type = TokenType::KEYWORDS[text]
    type = TokenType::IDENTIFIER if type.nil?
    addToken(type)
  end
end
