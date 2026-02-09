: asdf 2dup drop ;

: wawa  [`] thing ht@ ;
: wawa2 [`] thing ht! ;

0 varable #sym
: defsym define ['] docon , #sym @ , 1 #sym +! ;

: intern 2dup find 0= if #sym @ -rot defsym else nip nip >cfa >value @ ;

: ` word intern ;

: clear ` false swap ht! ;

(h) clear thing
(h) clear asdf
(h) clear qwer
