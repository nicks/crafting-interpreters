# Desc: error-handling code for rubylox.

module Err
  @@had_error = false

  def Err.report(line, where, message)
    STDERR.puts "[line #{line}] Error #{where}: #{message}"
    @@had_error = true
  end

  def Err.error(line, message)
    Err::report(line, "", message)
  end

  def Err.had_error
    @@had_error
  end
end
