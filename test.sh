TIMES="120 300 600 1800 3600 7200"
# 10800 14400 18000"
configs="validate-fuzz validate-fuzz,mutate-in-middle"


# set the same seed for all configs
RANDOM=0
MAX_ITERATIONS=1000000
arch="x86_64-apple-darwin"

# To limit the number of inputs
# -runs=1000000 
#  This:
# - Probe a number of inputs lower than the limit, trigger the exit code path, save the reproducer and exit.
# - Probe 1000000 inputs without breaking the checks and exit.

for time in $TIMES
do
    for config in $configs
    do
     
        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing corpus"
            rm -rf fuzz/corpus
            ex=$?
        done
        
        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing artifacts"
            rm -rf fuzz/artifacts
            ex=$?
        done


        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing coverage"
            rm -rf fuzz/coverage
            ex=$?
        done

        echo "Running $time $config"
        # Compile and run fuzz fir 1 iteration, this will prevent the real measurement of being obfuscated by the compiling process (warmup)
        cargo +nightly --verbose fuzz run egraphs-coverage --features $config -- -seed=0 -runs=1000 # 2> /dev/null
        # Remove previous corpus
        
        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing corpus"
            rm -rf fuzz/corpus
            ex=$?
        done
        
        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing artifacts"
            rm -rf fuzz/artifacts
            ex=$?
        done


        ex=1
        while [ $ex -gt 0 ]
        do
            echo "Removing coverage"
            rm -rf fuzz/coverage
            ex=$?
        done

        #cargo +nightly build -p fuzz --features $config
        #exit 1
        echo "Running fuzz"
        timeout --foreground $time cargo +nightly --verbose fuzz run egraphs-coverage --features $config -- -seed=0   2> $config.$time.logs

        echo "Getting coverage info"
        cargo +nightly fuzz coverage egraphs-coverage --features $config 2> /dev/null

        echo "Merging profdata"
        llvm-profdata merge -sparse fuzz/coverage/egraphs-coverage/raw -o fuzz/coverage/egraphs-coverage/coverage.profdata

        echo "Exporting profdata"
        llvm-cov export target/$arch/release/egraphs-coverage --format=text --instr-profile=fuzz/coverage/egraphs-coverage/coverage.profdata  > index.$config.$time.json

        llvm-cov show target/$arch/release/egraphs-coverage --format=html --instr-profile=fuzz/coverage/egraphs-coverage/coverage.profdata  > index.$config.$time.html

        # then run the coverage file
        # exit 1
        
        sleep 30
        echo "Saving data"
        zip -r data.$config.$time.zip fuzz/corpus fuzz/artifacts fuzz/coverage
    done
done