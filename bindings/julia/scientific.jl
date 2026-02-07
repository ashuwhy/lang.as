module ASJulia

using LinearAlgebra

function matrix_eigenvals(matrix::Matrix{Float64})
    return eigvals(matrix)
end

function matrix_svd(matrix::Matrix{Float64})
    return svd(matrix)
end

function matrix_solve(A::Matrix{Float64}, b::Vector{Float64})
    return A \ b
end

end 