use std::borrow::Cow;

use indexmap::IndexMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, FieldDef, FnSig, TyCtxt, VariantDef, Visibility};
use rustc_span::Symbol;

use crate::ReflectionStrategy;

#[derive(Debug)]
pub struct GenerationCandidate<'tcx, D> {
    pub did: Option<DefId>,
    pub friendly_name: Option<String>,
    pub def: D,
    pub variants: Vec<VariantCandidate<'tcx>>,
    pub excluded_variants: Vec<VariantCandidate<'tcx>>,
    pub trait_impls: IndexMap<DefId, Vec<DefId>>,
    pub notes: Vec<GenerationExclusionNote>,
    pub functions: Vec<FunctionCandidate<'tcx>>,
    pub excluded_functions: Vec<FunctionCandidate<'tcx>>,
}

impl<'tcx, D> GenerationCandidate<'tcx, D> {
    pub fn promote(self, def: AdtDef<'tcx>) -> GenerationCandidate<'tcx, AdtDef<'tcx>> {
        GenerationCandidate {
            def,
            friendly_name: self.friendly_name,
            did: Some(def.did()),
            variants: self.variants,
            notes: self.notes,
            excluded_variants: self.excluded_variants,
            trait_impls: self.trait_impls,
            functions: self.functions,
            excluded_functions: self.excluded_functions,
        }
    }
}

impl<D> Default for GenerationCandidate<'_, D>
where
    D: Default,
{
    fn default() -> Self {
        Self {
            friendly_name: Default::default(),
            def: Default::default(),
            variants: Default::default(),
            notes: Default::default(),
            did: Default::default(),
            excluded_variants: Default::default(),
            trait_impls: Default::default(),
            functions: Default::default(),
            excluded_functions: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCandidate<'tcx> {
    pub(crate) fn_name: Symbol,
    pub(crate) sig: FnSig<'tcx>,
    pub(crate) did: DefId,
    pub(crate) visibility: Visibility<DefId>,
    pub(crate) has_self: bool,
    pub(crate) is_unsafe: bool,
    pub(crate) kind: FunctionCandidateKind,
    pub(crate) notes: Vec<GenerationExclusionNote>,
    pub(crate) arguments: Vec<FunctionArgCandidate>,
    pub(crate) ret: FunctionArgCandidate,
}

#[derive(Debug)]
pub enum FunctionCandidateKind {
    TraitImplMethod { trait_did: DefId, impl_did: DefId },
    Method { impl_did: DefId },
}

impl FunctionCandidateKind {
    pub fn impl_did(&self) -> DefId {
        match self {
            FunctionCandidateKind::TraitImplMethod { impl_did, .. }
            | FunctionCandidateKind::Method { impl_did } => *impl_did,
        }
    }

    pub fn as_trait_fn(&self) -> Option<(DefId, DefId)> {
        if let FunctionCandidateKind::TraitImplMethod {
            trait_did,
            impl_did,
        } = self
        {
            Some((*trait_did, *impl_did))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct FunctionArgCandidate {
    pub friendly_name: String,
    pub reflection_strategy: ReflectionStrategy,
    pub notes: Vec<GenerationExclusionNote>,
}

impl FunctionArgCandidate {
    pub fn new(friendly_name: String) -> Self {
        Self {
            friendly_name,
            reflection_strategy: Default::default(),
            notes: Default::default(),
        }
    }
}

impl FunctionArgCandidate {
    pub fn with_reflection_strategy(mut self, strategy: ReflectionStrategy) -> Self {
        self.reflection_strategy = strategy;
        self
    }
}

#[derive(Debug)]
pub struct VariantCandidate<'tcx> {
    pub def: &'tcx VariantDef,
    pub fields: Vec<FieldCandidate>,
    pub excluded_fields: Vec<FieldCandidate>,
    pub notes: Vec<GenerationExclusionNote>,
}

impl<'tcx> VariantCandidate<'tcx> {
    pub fn new(def: &'tcx VariantDef) -> Self {
        Self {
            def,
            fields: Default::default(),
            excluded_fields: Default::default(),
            notes: Default::default(),
        }
    }
}
#[derive(Debug)]
pub struct FieldCandidate {
    pub did: DefId,
    pub name: Symbol,
    // pub vis: Visibility<DefId>,
    // pub safety: Safety,
    pub notes: Vec<GenerationExclusionNote>,
    pub reflection_strategy: ReflectionStrategy,
}

impl FieldCandidate {
    pub fn new(def: &FieldDef) -> Self {
        Self {
            did: def.did,
            name: def.name,
            // vis: def.vis,
            // safety: def.safety,
            notes: Default::default(),
            reflection_strategy: ReflectionStrategy::Filtered,
        }
    }

    pub fn with_reflection_strategy(mut self, strategy: ReflectionStrategy) -> Self {
        self.reflection_strategy = strategy;
        self
    }
}

#[derive(Debug, Clone)]
pub enum GenerationExclusionNote {
    Reason(String),
    #[allow(dead_code)]
    Section(&'static str),
}

impl std::fmt::Display for GenerationExclusionNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Excluded: ")?;
        match self {
            GenerationExclusionNote::Reason(reason) => f.write_str(reason.as_str())?,
            GenerationExclusionNote::Section(section) => f.write_str(section)?,
        }
        Ok(())
    }
}

pub trait Annotated {
    fn with_note(self, note: GenerationExclusionNote) -> Self;
    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote>;
    fn friendly_name<'a, 'tcx>(&'a self, tcx: TyCtxt<'tcx>) -> Cow<'a, str>
    where
        'tcx: 'a;
}

impl<D> Annotated for GenerationCandidate<'_, D> {
    fn with_note(mut self, note: GenerationExclusionNote) -> Self {
        self.notes.push(note);
        self
    }

    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote> {
        self.notes
            .iter()
            .chain(self.variants.iter().flat_map(|v| v.applying_notes()))
            .chain(
                self.excluded_variants
                    .iter()
                    .flat_map(|v| v.applying_notes()),
            )
            .chain(self.functions.iter().flat_map(|f| f.applying_notes()))
            .chain(
                self.excluded_functions
                    .iter()
                    .flat_map(|f| f.applying_notes()),
            )
    }
    fn friendly_name<'a, 'tcx>(&'a self, tcx: TyCtxt<'tcx>) -> Cow<'a, str>
    where
        'tcx: 'a,
    {
        self.did
            .map(|did| format!("{}::{}", tcx.crate_name(did.krate), tcx.item_name(did)).into())
            .unwrap_or(Cow::Borrowed("unknown"))
    }
}

impl Annotated for VariantCandidate<'_> {
    fn with_note(mut self, note: GenerationExclusionNote) -> Self {
        self.notes.push(note);
        self
    }

    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote> {
        self.notes
            .iter()
            .chain(self.fields.iter().flat_map(|f| f.applying_notes()))
            .chain(self.excluded_fields.iter().flat_map(|f| f.applying_notes()))
    }

    fn friendly_name<'a, 'tcx>(&'a self, _tcx: TyCtxt<'tcx>) -> Cow<'a, str>
    where
        'tcx: 'a,
    {
        self.def.name.as_str().into()
    }
}

impl Annotated for FieldCandidate {
    fn with_note(mut self, note: GenerationExclusionNote) -> Self {
        self.notes.push(note);
        self
    }

    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote> {
        self.notes.iter()
    }

    fn friendly_name<'a, 'tcx>(&'a self, _tcx: TyCtxt<'tcx>) -> Cow<'a, str>
    where
        'tcx: 'a,
    {
        self.name.as_str().into()
    }
}

impl<'tcx> Annotated for FunctionCandidate<'tcx> {
    fn with_note(mut self, note: GenerationExclusionNote) -> Self {
        self.notes.push(note);
        self
    }

    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote> {
        self.notes
            .iter()
            .chain(self.arguments.iter().flat_map(|a| a.applying_notes()))
            .chain(self.ret.applying_notes())
    }

    fn friendly_name<'a, 'b>(&'a self, _tcx: TyCtxt<'b>) -> Cow<'a, str>
    where
        'b: 'a,
    {
        self.fn_name.as_str().into()
    }
}

impl Annotated for FunctionArgCandidate {
    fn with_note(mut self, note: GenerationExclusionNote) -> Self {
        self.notes.push(note);
        self
    }

    fn applying_notes(&self) -> impl Iterator<Item = &GenerationExclusionNote> {
        self.notes.iter()
    }

    fn friendly_name<'a, 'tcx>(&'a self, _tcx: TyCtxt<'tcx>) -> Cow<'a, str>
    where
        'tcx: 'a,
    {
        (&self.friendly_name).into()
    }
}

pub struct AnnotationContextCollector<'n, 'tcx> {
    notes_with_context: Vec<(String, &'n GenerationExclusionNote)>,
    ctxt: TyCtxt<'tcx>,
}

impl<'n, 'tcx> AnnotationContextCollector<'n, 'tcx> {
    pub fn new(ctxt: TyCtxt<'tcx>) -> Self {
        Self {
            notes_with_context: Default::default(),
            ctxt,
        }
    }

    pub fn build(self) -> Vec<(String, &'n GenerationExclusionNote)> {
        self.notes_with_context
    }

    pub fn annotate<D>(&mut self, candidate: &'n GenerationCandidate<'tcx, D>) {
        let ctxt = candidate.friendly_name(self.ctxt);

        for note in &candidate.notes {
            self.notes_with_context.push((ctxt.to_string(), note))
        }

        for variant in &candidate.excluded_variants {
            self.annotate_variant(&ctxt, variant);
        }

        for variant in &candidate.variants {
            self.annotate_variant(&ctxt, variant);
        }

        for function in &candidate.excluded_functions {
            self.annotate_function(&ctxt, function);
        }
    }

    pub fn annotate_variant(&mut self, ctxt: &str, variant: &'n VariantCandidate<'tcx>) {
        let ctxt = format!("{ctxt}::{}", variant.friendly_name(self.ctxt));

        for note in &variant.notes {
            self.notes_with_context.push((ctxt.to_string(), note))
        }

        for field in &variant.fields {
            self.annotate_field(&ctxt, field)
        }

        for field in &variant.excluded_fields {
            self.annotate_field(&ctxt, field)
        }
    }

    pub fn annotate_field(&mut self, ctxt: &str, field: &'n FieldCandidate) {
        let ctxt = format!("{ctxt} {} (field)", field.friendly_name(self.ctxt));
        for note in &field.notes {
            self.notes_with_context.push((ctxt.to_string(), note))
        }
    }

    fn annotate_function(&mut self, ctxt: &str, function: &'n FunctionCandidate<'tcx>) {
        let ctxt = format!("{ctxt} fn {}", function.friendly_name(self.ctxt));

        for note in &function.notes {
            self.notes_with_context.push((ctxt.to_string(), note))
        }

        for arg in &function.arguments {
            self.annotate_arg(&ctxt, arg);
        }

        self.annotate_arg(&ctxt, &function.ret)
    }

    fn annotate_arg(&mut self, ctxt: &str, arg: &'n FunctionArgCandidate) {
        let ctxt = format!("{ctxt} {} (arg)", arg.friendly_name(self.ctxt));

        for note in &arg.notes {
            self.notes_with_context.push((ctxt.to_string(), note))
        }
    }
}
