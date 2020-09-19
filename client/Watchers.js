// ok
/** @jsx jsx */
import { jsx } from '@emotion/core';
import * as React from 'react';
import makeHandlers from './handlers';

const rm = (obj, k) => {
    delete obj[k];
    return obj;
};

const Watch = ({ head, name, runtime, config, unWatch }) => {
    const [result, setResult] = React.useState(null);

    const canRun =
        config.args.length == 0 ||
        config.args.every(
            (v, i) => v == null || config.values[i] !== undefined,
        );
    const output = React.useRef(null);

    const handlers = React.useRef(null);
    React.useEffect(() => {
        if (output.current && runtime) {
            handlers.current = makeHandlers(runtime, output.current);
        }
    }, [output.current, !!runtime]);

    const hasRun = React.useRef(false);

    React.useEffect(() => {
        if (!canRun || !handlers.current || !runtime) {
            console.log('not ready', config);
            return;
        }
        // TODO: when updating config values, need to clear out hasRun.
        // maybe change the name to "needsRun"
        if (hasRun.current) {
            return;
        }
        hasRun.current = true;
        if (
            config.args.length === 0 ||
            runtime.canRunSync(name, handlers.current)
        ) {
            setResult(runtime.runSync(name, config.values, handlers.current));
        } else {
            runtime.run(name, config.values, handlers.current);
        }
    }, [!!runtime, canRun, handlers.current, config]);
    return (
        <div>
            {name}
            <div ref={output} />
            {canRun ? <div>{JSON.stringify(result)}</div> : null}
        </div>
    );
};

const Watchers = ({ state, setState }) => {
    return (
        <div css={{ flex: 1, overflow: 'auto' }}>
            <h3>Watchers</h3>
            {Object.keys(state.watchers)
                .filter((k) => state.watchers[k])
                .map((key) => {
                    return (
                        <Watch
                            key={key}
                            name={key}
                            head={state.head}
                            runtime={state.runtime}
                            config={state.watchers[key]}
                            unWatch={() =>
                                setState((state) => ({
                                    ...state,
                                    watchers: rm(
                                        {
                                            ...state.watchers,
                                        },
                                        key,
                                    ),
                                }))
                            }
                        />
                    );
                    // const config = state.watchers[watcher]
                })}
        </div>
    );
};

export default Watchers;
