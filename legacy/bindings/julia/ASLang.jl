module ASLang

using Libdl

# Load the shared library
const lib_path = joinpath(@__DIR__, "../../target/release/libaslang.dylib") # Adjust for OS (dll/so)
const lib = Libdl.dlopen(lib_path)

function execute(code::String)
    # Get function pointer
    as_execute_ptr = Libdl.dlsym(lib, :as_execute)
    as_free_string_ptr = Libdl.dlsym(lib, :as_free_string)
    
    # Call as_execute
    result_ptr = ccall(as_execute_ptr, Cstring, (Cstring,), code)
    
    if result_ptr == C_NULL
        return ""
    end
    
    # Convert C string to Julia string
    result = unsafe_string(result_ptr)
    
    # Free the C string
    ccall(as_free_string_ptr, Cvoid, (Cstring,), result_ptr)
    
    return result
end

end # module
