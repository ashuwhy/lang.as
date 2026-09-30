// A money transfer whose contract is proved, not tested.
type Cents = int where 0 <= it && it <= 1_000_000_000_000

type Account = { id: int, balance: Cents, frozen: bool }
type Moved = { from: Account, to: Account }

enum TransferError { Frozen, Insufficient(short: Cents) }

pub fn transfer(from: Account, to: Account, amount: Cents) -> Result<Moved, TransferError>
  requires from.id != to.id
  requires to.balance + amount <= 1_000_000_000_000
  ensures result is Ok(m) ==> m.from.balance == from.balance - amount
  ensures result is Ok(m) ==> m.to.balance == to.balance + amount
  ensures (result is Err(Frozen)) == (from.frozen || to.frozen)
{
  if from.frozen || to.frozen { return Err(Frozen) }
  if from.balance < amount { return Err(Insufficient(amount - from.balance)) }
  Ok({ from: { ...from, balance: from.balance - amount },
       to:   { ...to,   balance: to.balance + amount } })
}

fn main() uses io {
  let a = { id: 1, balance: 100, frozen: false }
  let b = { id: 2, balance: 0, frozen: false }
  match transfer(from: a, to: b, amount: 30) {
    Ok(m) => io.print("moved, new balance:", m.to.balance)
    Err(e) => io.print("transfer failed")
  }
}
