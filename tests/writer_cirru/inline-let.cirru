
if
  and false $ some? memory
  , memory $ let
      factory $ render-method props state
      result $ factory default-intent
    swap! memorization assoc component result
    , result
