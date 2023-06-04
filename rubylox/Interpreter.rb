require_relative './Expr.rb'

class RuntimeError < StandardError
  attr_reader :token, :message
  
  def initialize(token, message)
    @token = token
    @message = message
  end

  def to_s
    "#{@message}"
  end
end

class Interpreter
  include Visitor
  
  def interpret(expr)
    begin
      value = evaluate(expr)
      puts stringify(value)
    rescue RuntimeError => e
      Err.runtime_error(e)
    end
  end

  def stringify(object)
    return "nil" if object.nil?
    return object if object.is_a?(String)
    if object.is_a?(Numeric)
      text = object.to_s
      return text.to_i if text.end_with?(".0")
      return text
    end
    object.to_s
  end

  def visitLiteral(expr)
    expr.value
  end
  
  def visitGrouping(expr)
    evaluate(expr.expression)
  end

  def visitUnary(expr)
    right = evaluate(expr.right)
    case expr.operator.type
    when TokenType::MINUS
      checkNumberOperand(expr.operator, right)
      -right
    when TokenType::BANG then !isTruthy(right)
    else nil
    end
  end

  def isTruthy(object)
    return false if object.nil?
    return false if object.is_a?(FalseClass)
    true
  end

  def visitBinary(expr)
    op = expr.operator
    left = evaluate(expr.left)
    right = evaluate(expr.right)
    case op.type
    when TokenType::MINUS
      checkNumberOperands(op, left, right)
      left - right
    when TokenType::SLASH
      checkNumberOperands(op, left, right)
      left / right
    when TokenType::STAR
      checkNumberOperands(op, left, right)
      left * right
    when TokenType::PLUS
      if left.is_a?(Numeric) && right.is_a?(Numeric)
        left + right
      elsif left.is_a?(String) && right.is_a?(String)
        left + right
      else
        raise RuntimeError.new(op,
                               "Operands must be two numbers or two strings.")
      end
    when TokenType::GREATER 
      checkNumberOperands(op, left, right)
      left > right
    when TokenType::GREATER_EQUAL 
      checkNumberOperands(op, left, right)
      left >= right
    when TokenType::LESS 
      checkNumberOperands(op, left, right)
      left < right
    when TokenType::LESS_EQUAL 
      checkNumberOperands(op, left, right)
      left <= right
    when TokenType::BANG_EQUAL then !isEqual(left, right)
    when TokenType::EQUAL_EQUAL then isEqual(left, right)
    else nil
    end
  end

  def isEqual(a, b)
    return true if a.nil? && b.nil?
    return false if a.nil?
    a == b
  end

  def evaluate(expr)
    expr.accept(self)
  end

  def checkNumberOperands(operator, left, right)
    return if left.is_a?(Numeric) && right.is_a?(Numeric)
    raise RuntimeError.new(operator, "Operands must be numbers.")
  end

  def checkNumberOperand(operator, x)
    return if x.is_a?(Numeric)
    raise RuntimeError.new(operator, "Operand must be number.")
  end
end
