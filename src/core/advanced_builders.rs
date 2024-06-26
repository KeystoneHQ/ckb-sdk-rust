use alloc::vec::Vec;

use ckb_types::{constants, core, packed, prelude::*};


/// An advanced builder for [`TransactionView`].
///
/// Base on [`packed::TransactionBuilder`] but added lots of syntactic sugar.
///
/// [`TransactionView`]: struct.TransactionView.html
/// [`packed::TransactionBuilder`]: ../packed/struct.TransactionBuilder.html
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    
    pub version: packed::Uint32,
    
    pub cell_deps: Vec<packed::CellDep>,
    
    pub header_deps: Vec<packed::Byte32>,
    
    pub inputs: Vec<packed::CellInput>,
    
    pub outputs: Vec<packed::CellOutput>,
    
    pub witnesses: Vec<packed::Bytes>,
    
    pub outputs_data: Vec<packed::Bytes>,
}

impl TransactionBuilder {
    pub fn get_cell_deps(&self) -> &Vec<packed::CellDep> {
        &self.cell_deps
    }
    
    pub fn get_header_deps(&self) -> &Vec<packed::Byte32> {
        &self.header_deps
    }

    pub fn get_inputs(&self) -> &Vec<packed::CellInput> {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &Vec<packed::CellOutput> {
        &self.outputs
    }

    pub fn get_witnesses(&self) -> &Vec<packed::Bytes> {
        &self.witnesses
    }

    pub fn get_outputs_data(&self) -> &Vec<packed::Bytes> {
        &self.outputs_data
    }
}

/*
 * Implement std traits.
 */
impl ::core::default::Default for TransactionBuilder {
    fn default() -> Self {
        Self {
            version: constants::TX_VERSION.pack(),
            cell_deps: Default::default(),
            header_deps: Default::default(),
            inputs: Default::default(),
            outputs: Default::default(),
            witnesses: Default::default(),
            outputs_data: Default::default(),
        }
    }
}

macro_rules! def_setter_simple {
    (__add_doc, $prefix:ident, $field:ident, $type:ident, $comment:expr) => {
        #[doc = $comment]
        pub fn $field(mut self, v: packed::$type) -> Self {
            self.$prefix.$field = v;
            self
        }
    };
    (__add_doc, $field:ident, $type:ident, $comment:expr) => {
        #[doc = $comment]
        pub fn $field(mut self, v: packed::$type) -> Self {
            self.$field = v;
            self
        }
    };
    ($prefix:ident, $field:ident, $type:ident) => {
        def_setter_simple!(
            __add_doc,
            $prefix,
            $field,
            $type,
            concat!("Sets `", stringify!($prefix), ".", stringify!($field), "`.")
        );
    };
    ($field:ident, $type:ident) => {
        def_setter_simple!(
            __add_doc,
            $field,
            $type,
            concat!("Sets `", stringify!($field), "`.")
        );
    };
}

macro_rules! def_setter_for_vector {
    (
        $prefix:ident, $field:ident, $type:ident,
        $func_push:ident, $func_extend:ident, $func_set:ident,
        $comment_push:expr, $comment_extend:expr, $comment_set:expr,
    ) => {
        #[doc = $comment_push]
        pub fn $func_push(&mut self, v: $prefix::$type) -> &mut Self {
            self.$field.push(v);
            self
        }
        #[doc = $comment_extend]
        pub fn $func_extend<T>(&mut self, v: T) -> &mut Self
        where
            T: ::core::iter::IntoIterator<Item = $prefix::$type>,
        {
            self.$field.extend(v);
            self
        }
        #[doc = $comment_set]
        pub fn $func_set(&mut self, v: Vec<$prefix::$type>) -> &mut Self {
            self.$field = v;
            self
        }
    };
    ($prefix:ident, $field:ident, $type:ident, $func_push:ident, $func_extend:ident, $func_set:ident) => {
        def_setter_for_vector!(
            $prefix,
            $field,
            $type,
            $func_push,
            $func_extend,
            $func_set,
            concat!("Pushes an item into `", stringify!($field), "`."),
            concat!(
                "Extends `",
                stringify!($field),
                "` with the contents of an iterator."
            ),
            concat!("Sets `", stringify!($field), "`."),
        );
    };
    ($field:ident, $type:ident, $func_push:ident, $func_extend:ident, $func_set:ident) => {
        def_setter_for_vector!(packed, $field, $type, $func_push, $func_extend, $func_set);
    };
    (set_i, $field:ident, $type:ident, $func_push:ident, $func_extend:ident, $func_set:ident, $func_set_i: ident) => {
        def_setter_for_vector!(packed, $field, $type, $func_push, $func_extend, $func_set);

        pub fn $func_set_i(&mut self, i: usize, v: packed::$type) -> &mut Self {
            self.$field[i] = v;
            self
        }
    };
}

macro_rules! def_dedup_setter_for_vector {
    (
        $prefix:ident, $field:ident, $type:ident,
        $func_push:ident, $func_extend:ident,
        $comment_push:expr, $comment_extend:expr,
    ) => {
        #[doc = $comment_push]
        pub fn $func_push(&mut self, v: $prefix::$type) -> &mut Self {
            if !self.$field.contains(&v) {
                self.$field.push(v);
            }
            self
        }
        #[doc = $comment_extend]
        pub fn $func_extend<T>(&mut self, v: T) -> &mut Self
        where
            T: ::core::iter::IntoIterator<Item = $prefix::$type>,
        {
            v.into_iter().for_each(|item| {
                if !self.$field.contains(&item) {
                    self.$field.push(item);
                }
            });
            self
        }
    };
    ($prefix:ident, $field:ident, $type:ident, $func_push:ident, $func_extend:ident) => {
        def_dedup_setter_for_vector!(
            $prefix,
            $field,
            $type,
            $func_push,
            $func_extend,
            concat!(
                "Pushes an item into `",
                stringify!($field),
                "` only if the same item is not already in."
            ),
            concat!(
                "Extends `",
                stringify!($field),
                "` with the contents of an iterator, skip already exist ones."
            ),
        );
    };
    ($field:ident, $type:ident, $func_push:ident, $func_extend:ident) => {
        def_dedup_setter_for_vector!(packed, $field, $type, $func_push, $func_extend);
    };
}

impl TransactionBuilder {
    def_setter_simple!(version, Uint32);
    def_setter_for_vector!(cell_deps, CellDep, cell_dep, cell_deps, set_cell_deps);
    def_dedup_setter_for_vector!(cell_deps, CellDep, dedup_cell_dep, dedup_cell_deps);
    def_setter_for_vector!(
        header_deps,
        Byte32,
        header_dep,
        header_deps,
        set_header_deps
    );

    def_dedup_setter_for_vector!(header_deps, Byte32, dedup_header_dep, dedup_header_deps);
    def_setter_for_vector!(inputs, CellInput, input, inputs, set_inputs);
    def_setter_for_vector!(
        set_i,
        outputs,
        CellOutput,
        output,
        outputs,
        set_outputs,
        set_output
    );
    def_setter_for_vector!(
        set_i,
        witnesses,
        Bytes,
        witness,
        witnesses,
        set_witnesses,
        set_witness
    );
    def_setter_for_vector!(
        set_i,
        outputs_data,
        Bytes,
        output_data,
        outputs_data,
        set_outputs_data,
        set_output_data
    );

    /// Converts into [`TransactionView`](struct.TransactionView.html).
    pub fn build(self) -> core::TransactionView {
        let Self {
            version,
            cell_deps,
            header_deps,
            inputs,
            outputs,
            witnesses,
            outputs_data,
        } = self;
        let raw = packed::RawTransaction::new_builder()
            .version(version)
            .cell_deps(cell_deps.pack())
            .header_deps(header_deps.pack())
            .inputs(inputs.pack())
            .outputs(outputs.pack())
            .outputs_data(outputs_data.pack())
            .build();
        let tx = packed::Transaction::new_builder()
            .raw(raw)
            .witnesses(witnesses.pack())
            .build();

        tx.into_view()
    }
}


pub fn convert_transaction_builder(tx: packed::Transaction) -> TransactionBuilder {
    return TransactionBuilder::default()
    .version(tx.raw().version())
    .cell_deps(tx.raw().cell_deps())
    .header_deps(tx.raw().header_deps())
    .inputs(tx.raw().inputs())
    .outputs(tx.raw().outputs())
    .outputs_data(tx.raw().outputs_data())
    .witnesses(tx.witnesses()).clone();
}

// impl AsTransactionBuilder for packed::Transaction {
//     /// Creates an advanced builder base on current data.
//     fn as_advanced_builder(&self) -> TransactionBuilder {
//         TransactionBuilder::default()
//             .version(self.raw().version())
//             .cell_deps(self.raw().cell_deps())
//             .header_deps(self.raw().header_deps())
//             .inputs(self.raw().inputs())
//             .outputs(self.raw().outputs())
//             .outputs_data(self.raw().outputs_data())
//             .witnesses(self.witnesses())
//     }
// }