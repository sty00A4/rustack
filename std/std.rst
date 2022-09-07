macro clear( LENGTH repeat( pop ) )

macro[1] pop (@a)
macro[1] dup ( {a}      a a   )
macro[2] swap( {a b}    b a   )
macro[2] over( {a b}    a b a )
macro[3] rot ( {a b c}  b c a )