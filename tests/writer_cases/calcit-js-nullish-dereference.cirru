
'main! $ %{} 'CodeEntry
  :doc "|A nullable host value must be narrowed before member access."
  :code $ quote $ defn main! ()
    let
        host $ js/process.argv
      .-length host
  :examples $ []
  :schema $ :: 'Fn $ {} (:return 'Number)
    :args $ []
    :features $ #{} :js-ffi
