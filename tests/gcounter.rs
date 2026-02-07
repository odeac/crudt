use crudt::gcounter::GCounter;

#[test]
fn gcounter_starts_at_zero() {
    let c = GCounter::new();
    assert_eq!(c.value(), 0);
}

#[test]
fn gcounter_can_be_incremented() {
    let mut c = GCounter::new();
    c.inc(1);
    c.inc(2);
    assert_eq!(c.value(), 3);
}

#[test]
fn gcounter_merge_is_commutative() {
    let mut a = GCounter::new();
    let mut b = GCounter::new();

    a.inc(3);
    b.inc(5);

    let mut ab = a.clone();
    ab.merge(b.clone());

    let mut ba = b;
    ba.merge(a);

    assert_eq!(ab.value(), ba.value());
}

#[test]
fn gcounter_merge_is_idempotent() {
    let mut a = GCounter::new();
    a.inc(10);

    let mut b = a.clone();
    b.merge(a.clone());

    assert_eq!(b.value(), a.value());
}

#[test]
fn gcounter_independent_increments_add_up_after_merge() {
    let mut a = GCounter::new();
    let mut b = GCounter::new();

    a.inc(5);
    b.inc(3);

    let mut a2 = a.clone();
    a2.merge(b.clone());

    let mut b2 = b;
    b2.merge(a);

    assert_eq!(a2.value(), 8);
    assert_eq!(b2.value(), 8);
}
