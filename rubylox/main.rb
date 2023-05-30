# Desc: Main entry point for rubylox.

require "./error.rb"
require "./Scanner.rb"

# Run from a string.
def run(source)
  scanner = Scanner.new(source)
  tokens = scanner.scanTokens()

  if Err.had_error
    exit 65
  end
  
  # For now, just print the tokens.
  tokens.each do |token|
    puts token
  end
end

# Run from a file.
def runFile(filename)
  unless File.exist?(filename)
    raise "File does not exist: #{filename}"
  end
  run(File.read(filename))
end

# Run from stdin.
def runPrompt()
  loop do
    print "> "
    begin
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
