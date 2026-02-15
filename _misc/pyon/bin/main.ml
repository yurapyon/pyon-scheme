(* tokens *)

let is_whitespace ch = ch == ' ' || ch == '\n'
let is_break ch = is_whitespace ch || ch == '(' || ch == ')'

let next_char input index =
  if index >= String.length input
  then None, index
  else Some input.[index], index + 1
;;

let rec skip_whitespace input index =
  match next_charinput index with
  | Some ch, i when is_whitespace ch -> skip_whitespace input i
  | Some _, i -> i - 1
  | None, i -> i
;;

let next_token input index =
  let rec next_token_ index is_symbol =
    match next_char input index with
    | Some ch, i when is_break ch -> if is_symbol then i - 1 else i
    | Some _, i -> next_token_ i true
    | None, i -> i
  in
  let start = skip_whitespace input index in
  let end_ = next_token_ start false in
  if start == end_
  then None, end_
  else Some (String.sub input start (end_ - start)), end_
;;

(* parser *)

type value =
  | Nil
  | Integer of int
  | Word of string
  | Builtin of unit
  | Lambda of lambda
  | Cons of cons

and cons = value * value
and lambda = value * value

type parser =
  { root : value
  ; ast_stack : Stack.t
  }

let parse token p =
  let append value p =
    let nv = Cons (Nil, Nil) in
    0
  in
  match token with
  | Some str when str == "(" -> 0
  | Some str when str == ")" -> p.ast_stack <- Stack.drop ast_stack
  | Some str -> append Word str
  | None -> p
;;

(*
let parse token root ast_stack =
  let rec parse_ token ast_stack =
    let append v =
      let nc = Cons (Nil, Nil) in
      ast_stack |> Stack.drop |> Stack.push Cons (v, nc)
    in
    match token with
    | Some str when str == "(" -> 0
    | Some str when str == ")" -> Stack.drop ast_stack
    | Some str -> append Word str
    | None -> ast_stack
  in
  parse_ token ast_stack
;;
 *)

(* main *)

let () =
  let str = "  (hello   world\n)" in
  let print index =
    let nt = next_token str index in
    match nt with
    | Some token, i ->
      Printf.printf "%s\n" token;
      i
    | None, i -> i
  in
  0 |> print |> print |> print |> print |> print |> print |> ignore
;;
