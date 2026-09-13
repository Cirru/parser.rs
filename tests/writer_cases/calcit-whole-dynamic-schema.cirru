
|main! $ %{} 'CodeEntry (:doc "|Entry that makes open-identity reachable.")
  :code $ quote $ defn main! () (open-identity 1)
  :examples $ []
  :schema $ :: 'Fn $ {} (:return 'Number)
    :args $ []

|reload! $ %{} 'CodeEntry (:doc "|Reload handler.")
  :code $ quote $ defn reload! () &unit
  :examples $ []
  :schema $ :: 'Fn $ {} (:return 'Unit)
    :args $ []
