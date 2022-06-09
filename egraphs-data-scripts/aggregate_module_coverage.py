import json
import sys
import re

def read_input_count(f):
    content = open(f, 'r')

    r = re.compile(r"(\d+) NEW ")

    for line in content.readlines()[::-1]:
        print(line)
        if "NEW" in line:
            return int(line.split("NEW")[0][1::])

if __name__ == "__main__":
    module_filter = sys.argv[1]
    out = sys.argv[2]
    # multiple data files to register all as a time serie
    data_files = sys.argv[3::2]
    logs = sys.argv[4::2]

    cumulative = {
        module_filter: dict(
            branches=dict(
                total=[],
                covered=[]
            ),
            lines = dict(
                total=[],
                covered = []
            ),
            input_count = [],
            time = []
        )
    }
    # load the files
    for file, log in zip(data_files, logs):
        content = open(file, 'r').read()
        data = json.loads(content)['data'][0]

        
        cumulative[module_filter]['branches']['total'].append(0)
        cumulative[module_filter]['branches']['covered'].append(0)
        cumulative[module_filter]['lines']['total'].append(0)
        cumulative[module_filter]['lines']['covered'].append(0)

        inputs_count = read_input_count(log)
        cumulative[module_filter]['input_count'].append(inputs_count)
        time = int(file.split(".")[-2])
        cumulative[module_filter]['time'].append(time)


        #print(data.keys())
        for file in data['files']:
            if module_filter in file['filename']:
                cumulative[module_filter]['branches']['total'][-1] += file['summary']['branches']['count']
                cumulative[module_filter]['branches']['covered'][-1] += file['summary']['branches']['covered']
                
                cumulative[module_filter]['lines']['total'][-1] += file['summary']['lines']['count']
                cumulative[module_filter]['lines']['covered'][-1] += file['summary']['lines']['covered']
                

    print(cumulative)
    open(out, 'w').write(json.dumps(cumulative, indent=4))
    # traverse it and aggregate the module