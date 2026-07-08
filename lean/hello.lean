structure PPoint (α : Type) where
  x : α
  y : α

def natOrigin : PPoint Nat :=
  { x := Nat.zero, y := Nat.zero }

def List.my_head? {α : Type} (xs : List α) : Option α :=
  match xs with
  | [] => none
  | y :: _ => some y

def last? {α : Type} (xs : List α) : Option α :=
  match xs with
  | [] => none
  | y :: [] => some y
  | _ :: ys => last? ys

def l : List Nat := []

#eval last? l

def List.findFirst? {α : Type} (xs : List α) (predicate : α -> Bool) : Option α :=
  match xs with
  | [] => none
  | y :: ys =>
    if predicate y
    then y
    else List.findFirst? ys predicate

def isEven (n : Nat) : Bool :=
  n % 2 = 0

#eval List.findFirst? [1,5,2,3] isEven

def Prod.switch {α β : Type} (pair : α × β) : β × α :=
  (pair.snd, pair.fst)

#eval Prod.switch (1, 'a')

inductive PetName where
  | cat (name: String) : PetName
  | dog (name: String) : PetName

def catName : PetName := PetName.cat "Catty"

#eval catName

-- !echo -e "\u2295"
-- ⊕ u2295
-- × u00D7

def zip {α β : Type} (xs : List α) (ys : List β) : List (α × β) :=
  match xs with
  | [] => []
  | x :: xs =>
    match ys with
    | [] => []
    | y :: ys => (x, y) :: zip xs ys

#eval zip [1,2,3] [4,5]

def take {α : Type} (n : Nat) (xs: List α) : List α :=
  if n = 0
  then
    []
  else
    match xs with
    | [] => []
    | x :: xs => x :: take (n - 1) xs

#eval take 3 ["a", "b"]

def distribute {α β γ : Type} (x : α × (β ⊕ γ)) : (α × β) ⊕ (α × γ) :=
  let a := x.fst
  let b := x.snd
  match b with
  | Sum.inl bl => Sum.inl (a, bl)
  | Sum.inr br => Sum.inr (a, br)

def sumIn : (Nat × (String ⊕ Int)) := (1, Sum.inl "2")

#eval distribute sumIn

def multType {α : Type} (a : Bool × α) : α ⊕ α :=
  let bool := a.fst
  let val := a.snd
  if bool
  then Sum.inr val
  else Sum.inl val

def boolVal : Bool × Nat := (true, 2)

#eval multType boolVal

def main : IO Unit := do
  IO.println "---"
 
