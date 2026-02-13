let next_char input index =
  if index >= String.length input then None, index else Some input.[index], index + 1
;;

let is_whitespace ch = ch == ' ' || ch == '\n'
let is_break ch = is_whitespace ch || ch == '(' || ch == ')'

let rec skip_whitespace input index =
  match next_char input index with
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
  match next_token_ start false with
  | i when i == start -> None, i
  | i -> Some (String.sub input start (i - start)), i
;;

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
