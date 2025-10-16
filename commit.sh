#library/backtrace library/stdarch src/doc/book doc/edition-guide
# src/gcc
for x in  src/doc/embedded-book src/doc/nomicon src/doc/reference src/doc/rust-by-example  src/llvm-project
do
    echo $x
    pushd $x
    git status
    git remote -v
    git branch
    popd
done
