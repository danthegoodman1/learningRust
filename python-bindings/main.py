import dan
import pyarrow as pa

arr = pa.array([1, 2, 3, 4])

print(dan.sum_as_string(1, 2))
dbld = dan.double_arr(arr)
print(dbld, type(dbld))
