//! Affine transformations on nodes.

use super::prelude::*;
use crate::geom::Transform;

/// Transform a node without affecting layout.
#[derive(Debug, Hash)]
pub struct TransformNode<T: TransformKind> {
    /// Transformation to apply to the contents.
    pub kind: T,
    /// The node whose contents should be transformed.
    pub child: LayoutNode,
}

#[class]
impl<T: TransformKind> TransformNode<T> {
    /// The origin of the transformation.
    pub const ORIGIN: Spec<Option<Align>> = Spec::default();

    fn construct(_: &mut EvalContext, args: &mut Args) -> TypResult<Template> {
        Ok(Template::inline(Self {
            kind: T::construct(args)?,
            child: args.expect("body")?,
        }))
    }
}

impl<T: TransformKind> Layout for TransformNode<T> {
    fn layout(
        &self,
        ctx: &mut LayoutContext,
        regions: &Regions,
        styles: StyleChain,
    ) -> Vec<Constrained<Arc<Frame>>> {
        let origin = styles.get(Self::ORIGIN).unwrap_or(Align::CENTER_HORIZON);
        let matrix = self.kind.matrix();

        let mut frames = self.child.layout(ctx, regions, styles);

        for Constrained { item: frame, .. } in &mut frames {
            let Spec { x, y } = origin.zip(frame.size).map(|(o, s)| o.resolve(s));
            let transform = Transform::translation(x, y)
                .pre_concat(matrix)
                .pre_concat(Transform::translation(-x, -y));

            Arc::make_mut(frame).transform(transform);
        }

        frames
    }
}

/// Kinds of transformations.
pub trait TransformKind: Debug + Hash + Sized + Sync + Send + 'static {
    fn construct(args: &mut Args) -> TypResult<Self>;
    fn matrix(&self) -> Transform;
}

/// A translation on the X and Y axes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move(pub Length, pub Length);

impl TransformKind for Move {
    fn construct(args: &mut Args) -> TypResult<Self> {
        let tx = args.named("x")?.unwrap_or_default();
        let ty = args.named("y")?.unwrap_or_default();
        Ok(Self(tx, ty))
    }

    fn matrix(&self) -> Transform {
        Transform::translation(self.0, self.1)
    }
}

/// A rotational transformation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rotate(pub Angle);

impl TransformKind for Rotate {
    fn construct(args: &mut Args) -> TypResult<Self> {
        Ok(Self(args.named_or_find("angle")?.unwrap_or_default()))
    }

    fn matrix(&self) -> Transform {
        Transform::rotation(self.0)
    }
}

/// A scale transformation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scale(pub Relative, pub Relative);

impl TransformKind for Scale {
    fn construct(args: &mut Args) -> TypResult<Self> {
        let all = args.find()?;
        let sx = args.named("x")?.or(all).unwrap_or(Relative::one());
        let sy = args.named("y")?.or(all).unwrap_or(Relative::one());
        Ok(Self(sx, sy))
    }

    fn matrix(&self) -> Transform {
        Transform::scale(self.0, self.1)
    }
}
