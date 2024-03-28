; ModuleID = 'dummy_module'
source_filename = "dummy_module"

define i1 @foo(i64 %0, i64 %1) {
entry:
  br i1 false, label %then, label %else

then:                                             ; preds = %entry

else:                                             ; preds = %entry
}
