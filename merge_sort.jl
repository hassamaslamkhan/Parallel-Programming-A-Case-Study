using Distributed
using SharedArrays

n = 5
addprocs(n)

@everywhere function merge_sort(x)
  if length(x) == 1
    return x
  end
  mid = div(length(x),2);
  left = @spawnat :any merge_sort(x[1:mid]);
  right = merge_sort(x[(mid+1):length(x)]);
  left = fetch(left)
  sorted = Array{Float32}(undef, length(x));
  i = 0;
  j = 0;
  while (i + j) < length(x)
    if (i < length(left) && j < length(right) && left[i+1] < right[j+1]) || j >= length(right)
      i += 1
      sorted[i+j] = left[i]
    else
      j += 1
      sorted[i+j] = right[j]
    end
  end
  return sorted
end

x = SharedArray(rand(1000000))
@time merge_sort(x);