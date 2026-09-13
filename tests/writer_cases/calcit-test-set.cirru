
|test-method-dispatch $ %{} 'CodeEntry (:doc |)
  :code $ quote $ defn test-method-dispatch ()
    assert= (#{} 1 2 3)
      .add (#{} 1 2) 3
  :examples $ []
  :schema $ :: 'Dynamic
