; ModuleID = 'mymodule'
source_filename = "mymodule"

define i1 @foo(i64 %0, i64 %1) {
entry:
  br i1 false, label %then, label %else

then:                                             ; preds = %entry
  ret i1 false

else:                                             ; preds = %entry
  ret i1 true
}
