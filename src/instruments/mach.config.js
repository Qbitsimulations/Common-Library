// eslint-disable-next-line strict
const imagePlugin = require('esbuild-plugin-inline-image');
const postCssPlugin = require('esbuild-style-plugin');
const tailwind = require('tailwindcss');
const postCssColorFunctionalNotation = require('postcss-color-functional-notation');
// eslint-disable-next-line import/no-extraneous-dependencies
const postCssInset = require('postcss-inset');

/** @type { import('@synaptic-simulations/mach').MachConfig } */
module.exports = {
    packageName: 'Q737',
    packageDir: 'build/qbit-aircraft-b737-800',
    plugins: [
        imagePlugin({ limit: -1 }),
        postCssPlugin({
            extract: true,
            postcss: {
                plugins: [
                    tailwind('tailwind.config.js'),

                    // transform: hsl(x y z / alpha) -> hsl(x, y, z, alpha)
                    postCssColorFunctionalNotation(),

                    // transform: inset: 0; -> top/right/left/bottom: 0;
                    postCssInset()
                ]
            }
        })
    ],
    instruments: [
        reactInstrument('Example'),

    ]
};

function msfsAvionicsInstrument(name) {
    return {
        name,
        index: `qbt-737ec/src/instruments/src/${name}/instrument.tsx`,
        simulatorPackage: {
            type: 'baseInstrument',
            templateId: `Q737_${name}`,
            mountElementId: `${name}_CONTENT`,
            fileName: name.toLowerCase(),
            imports: ['/JS/dataStorage.js']
        }
    };
}

function reactInstrument(name, additionalImports) {
    return {
        name,
        index: `src/${name}/index.tsx`,
        simulatorPackage: {
            type: 'react',
            isInteractive: false,
            fileName: name.toLowerCase(),
            imports: ['/JS/dataStorage.js', '/JS/Q737EC_Simvars.js', ...(additionalImports ?? [])]
        }
    };
}