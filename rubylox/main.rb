# Desc: Main entry point for rubylox.

require_relative "./error.rb"
require_relative "./Scanner.rb"
require_relative "./Parser.rb"
require_relative "./Interpreter.rb"

# Run from a string.
def run(source)
  scanner = Scanner.new(source)
  tokens = scanner.scanTokens()

  if Err.had_error
    return
  end

  parser = Parser.new(tokens)
  stmts = parser.parse()
  
  if Err.had_error
    return
  end

  interpreter = Interpreter.new()
  interpreter.interpret(stmts)
end

# Run from a file.
def runFile(filename)
  unless File.exist?(filename)
    raise "File does not exist: #{filename}"
  end
  run(File.read(filename))

  if Err.had_error
    exit(65)
  end
  if Err.had_runtime_error
    exit(70)
  end
end

# Run from stdin.
def runPrompt()
  loop do
    print "> "
    begin
      Err.reset()
      run(ARGF.readline)
    rescue EOFError
      break
    end
  end      
end

def main
  if ARGV.length > 1
    puts "Usage: rubylox [script]"
    exit
  elsif ARGV.length == 1
    runFile(ARGV[0])
  else
    runPrompt()
  end
end
