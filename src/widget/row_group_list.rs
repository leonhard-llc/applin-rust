#![allow(clippy::many_single_char_names)]
use crate::widget::RowList;

/// This struct converts a tuple of row group builders (`Into<RowList>`) to a vector of row groups.
/// It supports tuples of length 0 through 20.
pub struct RowGroupList(pub Vec<RowList>);

impl RowGroupList {
    #[must_use]
    pub fn to_vec(self) -> Vec<RowList> {
        self.0
    }
}

impl From<Vec<RowList>> for RowGroupList {
    fn from(v: Vec<RowList>) -> Self {
        Self(v)
    }
}

// From tuples of length 0 through 20.
impl From<()> for RowGroupList {
    fn from((): ()) -> Self {
        RowGroupList(vec![])
    }
}

impl<A: Into<RowList>> From<(A,)> for RowGroupList {
    fn from((a,): (A,)) -> Self {
        RowGroupList(vec![a.into()])
    }
}

impl<A: Into<RowList>, B: Into<RowList>> From<(A, B)> for RowGroupList {
    fn from((a, b): (A, B)) -> Self {
        RowGroupList(vec![a.into(), b.into()])
    }
}

impl<A: Into<RowList>, B: Into<RowList>, C: Into<RowList>> From<(A, B, C)> for RowGroupList {
    fn from((a, b, c): (A, B, C)) -> Self {
        RowGroupList(vec![a.into(), b.into(), c.into()])
    }
}

impl<A: Into<RowList>, B: Into<RowList>, C: Into<RowList>, D: Into<RowList>> From<(A, B, C, D)>
    for RowGroupList
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        RowGroupList(vec![a.into(), b.into(), c.into(), d.into()])
    }
}

impl<A: Into<RowList>, B: Into<RowList>, C: Into<RowList>, D: Into<RowList>, E: Into<RowList>>
    From<(A, B, C, D, E)> for RowGroupList
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        RowGroupList(vec![a.into(), b.into(), c.into(), d.into(), e.into()])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
    > From<(A, B, C, D, E, F)> for RowGroupList
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
    > From<(A, B, C, D, E, F, G)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L)> for RowGroupList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (A, B, C, D, E, F, G, H, I, J, K, L, M),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
        P: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
        P: Into<RowList>,
        Q: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
        P: Into<RowList>,
        Q: Into<RowList>,
        R: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
        P: Into<RowList>,
        Q: Into<RowList>,
        R: Into<RowList>,
        S: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
            s.into(),
        ])
    }
}

impl<
        A: Into<RowList>,
        B: Into<RowList>,
        C: Into<RowList>,
        D: Into<RowList>,
        E: Into<RowList>,
        F: Into<RowList>,
        G: Into<RowList>,
        H: Into<RowList>,
        I: Into<RowList>,
        J: Into<RowList>,
        K: Into<RowList>,
        L: Into<RowList>,
        M: Into<RowList>,
        N: Into<RowList>,
        O: Into<RowList>,
        P: Into<RowList>,
        Q: Into<RowList>,
        R: Into<RowList>,
        S: Into<RowList>,
        T: Into<RowList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)> for RowGroupList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
        ),
    ) -> Self {
        RowGroupList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
            s.into(),
            t.into(),
        ])
    }
}
