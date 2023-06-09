# Desc: Main entry point for generate-ast
# A helper script to generate the AST classes.

def define_visitor(f, base_name, types)
  f.puts "module #{base_name}Visitor"
  types.each do |type|
    type_name = type.split(":")[0].strip
    f.puts "  def visit#{type_name}(#{base_name.downcase})"
    f.puts "    raise NotImplementedError"
    f.puts "  end"
    f.puts
  end
  f.puts "end"
  f.puts
end

def define_type(f, base_name, class_name, fields)
  names = fields.split(", ").map { |field| field.split(" ")[1] }
  f.puts "class #{class_name} < #{base_name}"
  f.puts "  attr_reader #{names.map { |name| ":#{name}" }.join(", ")}"
  f.puts "  def initialize(#{names.join(", ")})"
  names.each do |name|
    f.puts "    @#{name} = #{name}"
  end
  f.puts "  end"
  f.puts
  f.puts "  def accept(visitor)"
  f.puts "    visitor.visit#{class_name}(self)"
  f.puts "  end"
  f.puts
  f.puts "end"
  f.puts
end

def define_ast(output_dir, base_name, types)
  path = "#{output_dir}/#{base_name}.rb"
  File.open(path, "w") do |f|
    f.puts "# Path: #{path}"
    f.puts "# Desc: Auto-generated AST for #{base_name}"
    f.puts
    f.puts "class #{base_name}"
    f.puts "  def accept(visitor)"
    f.puts "    raise NotImplementedError"
    f.puts "  end"
    f.puts "end"
    f.puts
    define_visitor(f, base_name, types)
    f.puts

    # The AST classes.
    types.each do |type|
      class_name = type.split(":")[0].strip
      fields = type.split(":")[1].strip
      define_type(f, base_name, class_name, fields)
    end
  end
end

def main
  if ARGV.length != 1
    puts "Usage: generate-ast [output directory]"
    exit
  end

  output_dir = ARGV[0]
  define_ast(output_dir, "Expr", [
               "Binary   : Expr left, Token operator, Expr right",
               "Grouping : Expr expression",
               "Literal  : Object value",
               "Unary    : Token operator, Expr right",
               "Variable : Token name",
               "Assign   : Token name, Expr value",
               "Logical  : Expr left, Token operator, Expr right",
               "Call     : Expr callee, Token paren, Array<Expr> arguments",
               "Get      : Expr object, Token name",
               "SetExpr  : Expr object, Token name, Expr value",
               "Super    : Token keyword, Token method",
               "This     : Token keyword",
             ])
  define_ast(output_dir, "Stmt", [
               "ExprStmt  : Expr expression",
               "IfStmt    : Expr condition, Stmt then_branch, Stmt else_branch",
               "PrintStmt : Expr expression",
               "ReturnStmt: Token keyword, Expr value",
               "VarStmt   : Token name, Expr initializer",
               "BlockStmt : Array<Stmt> statements",
               "WhileStmt : Expr condition, Stmt body",
               "FunctionStmt : Token name, Array<Token> params, Array<Stmt> body",
               "ClassStmt : Token name, Variable superclass, Array<Stmt> methods",
             ])
  
end
