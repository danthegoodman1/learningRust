import pyarrow as pa
from dan import sum_as_string, double_arr

arr = pa.array([1, 2, 3, 4.1])
arr2 = pa.array([1, 2, 3, 4])
print(type(arr))
print(type(arr2))

print(sum_as_string(1, 2))
res = double_arr(arr)
res2 = double_arr(arr2)
print(print(res), type(res))
print(print(res2), type(res2))
