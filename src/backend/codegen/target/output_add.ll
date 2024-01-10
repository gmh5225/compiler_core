; ModuleID = 'test_module'
source_filename = "test_module"

define i64 @add(i64 %0, i64 %1) {
entry:
  %sum = add i64 %0, %1
  ret i64 %sum
}
