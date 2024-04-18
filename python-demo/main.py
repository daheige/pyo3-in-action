import string_utils;

x = string_utils.sum_as_string(12, 1);
print("x = ", x);

s = string_utils.explode("a,b,c", ",")
print("s = ", s);

arr = s = string_utils.implode(['a', 'b', 'c'], ",")
print("arr = ", arr);
