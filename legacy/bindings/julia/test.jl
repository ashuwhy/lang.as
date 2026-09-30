include("ASLang.jl")
using .ASLang

println("Running AS Lang from Julia...")
output = ASLang.execute("print(\"Hello from Julia!\"); let x = 10 * 10; print(x);")
println("Output:\n$output")
