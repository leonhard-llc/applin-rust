#![allow(clippy::many_single_char_names)]
use crate::widget::{OptWidgetList, Widget};
use serde::{Deserialize, Serialize};

/// This struct converts tuples of tuples of widget builders (`Into<OptWidgetList>`)
/// to a `Vec<Vec<Option<Widget>>>`.
/// It supports tuples of length 0 through 10.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RowList(pub Vec<Vec<Option<Widget>>>);
impl RowList {
    #[must_use]
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, widgets: impl Into<OptWidgetList>) {
        self.0.push(widgets.into().to_vec());
    }

    #[must_use]
    pub fn to_vec(self) -> Vec<Vec<Option<Widget>>> {
        self.0
    }
}

impl Default for RowList {
    fn default() -> Self {
        Self::new()
    }
}

// From tuples of length 0 through 20.
impl From<()> for RowList {
    fn from((): ()) -> Self {
        RowList(vec![])
    }
}

impl<A: Into<OptWidgetList>> From<(A,)> for RowList {
    fn from((a,): (A,)) -> Self {
        RowList(vec![a.into().0])
    }
}

impl<A: Into<OptWidgetList>, B: Into<OptWidgetList>> From<(A, B)> for RowList {
    fn from((a, b): (A, B)) -> Self {
        RowList(vec![a.into().0, b.into().0])
    }
}

impl<A: Into<OptWidgetList>, B: Into<OptWidgetList>, C: Into<OptWidgetList>> From<(A, B, C)>
    for RowList
{
    fn from((a, b, c): (A, B, C)) -> Self {
        RowList(vec![a.into().0, b.into().0, c.into().0])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
    > From<(A, B, C, D)> for RowList
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        RowList(vec![a.into().0, b.into().0, c.into().0, d.into().0])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
    > From<(A, B, C, D, E)> for RowList
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F)> for RowList
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G)> for RowList
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H)> for RowList
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I)> for RowList
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J)> for RowList
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K)> for RowList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L)> for RowList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for RowList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (A, B, C, D, E, F, G, H, I, J, K, L, M),
    ) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for RowList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    ) -> Self {
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
        S: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
            s.into().0,
        ])
    }
}

impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
        S: Into<OptWidgetList>,
        T: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)> for RowList
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
        RowList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
            s.into().0,
            t.into().0,
        ])
    }
}
