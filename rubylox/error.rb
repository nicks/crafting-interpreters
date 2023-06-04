# Desc: error-handling code for rubylox.

module Err
  @@had_error = false
  @@had_runtime_error = false

  def Err.report(line, where, message)
    STDERR.puts "[line #{line}] Error #{where}: #{message}"
    @@had_error = true
  end

  def Err.runtime_error(error)
    STDERR.puts "#{error.message}\n[line #{error.token.line}]"
    @@had_runtime_error = true
  end

  def Err.parse_error(token, message)
    if token.type == :EOF
      Err::report(token.line, " at end", message)
    else
      Err::report(token.line, " at '#{token.lexeme}'", message)
    end
  end

  def Err.error(line, message)
    Err::report(line, "", message)
  end

  def Err.had_error
    @@had_error
  end

  def Err.had_runtime_error
    @@had_runtime_error
  end
end
