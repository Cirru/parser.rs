
cond
    string? x
    pr-str x
  (bool? x)
    str x
  (symbol? x)
    str "\"\'" x
  (map? x) "\"a map"
  (set? x) "\"a set"
  true $ str
