# Update How Request Body of Post is Parsed

## - Check the method first

**Do not parse body to string.** Match on first new line byte, parse that
and keep the rest as bytes, as I am sure that serde will be able 
to serialize the bytes json object into whatever type.

**Doing it this way will save base64 encoding strings on the frontend 
and then compressing them.**

*Use the first line as the identifier for what type needs to be 
serialized.*
