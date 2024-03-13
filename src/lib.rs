///////////////////////////////////////////////////////////////////////////////

pub mod algorithms {

    pub mod search {
        pub mod binary_search;
        pub mod linear_search;
    }

    //.......................................................................//

    pub mod sort {
        pub mod selection_sort {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod insertion_sort {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod merge_sort;
        pub mod quick_sort;

        #[cfg(test)]
        mod shared_test_cases;
    }

    pub mod graphs;
}

//---------------------------------------------------------------------------//

pub mod data_structures {

    pub mod linked_list {
        pub mod single_linked_list {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod double_linked_list {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }

    //.......................................................................//

    pub mod stack {

        pub mod linked_stack {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod array_stack {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }

    //.......................................................................//

    pub mod queue {

        pub mod linked_queue {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod array_queue {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }

    pub mod binary_heap;
    pub mod graphs;
    pub mod maps;
    pub mod sets;
}

//---------------------------------------------------------------------------//

pub mod meta {
    pub mod benchmark;
}

///////////////////////////////////////////////////////////////////////////////
