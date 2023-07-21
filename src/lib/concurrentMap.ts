type CallBack<T, U> = (arg: T) => Promise<U>;


/**
 * 
 * @param inputArray Array of items to be processed
 * @param func Function to be called on each item
 * @param maxConcurrency Max amount of simultaneous pending operations
 * @returns Promise that resolves when all the functions have finished running
 */
export async function mapWithConcurrency<T, U>(inputArray: T[], func: CallBack<T, U>, maxConcurrency: number = 3): Promise<U[]> {

    const runningPromises: Promise<U | void>[] = [];
    const results: U[] = [];

    inputArray.forEach(async (item, i) => {
        const promise = func(item).then((result) => {
            results[i] = result;
            // remove promise from runningPromises
            const index = runningPromises.indexOf(promise);
            runningPromises.splice(index, 1);
        }).catch((err) => {
            console.error(err, item);
            // remove promise from runningPromises
            const index = runningPromises.indexOf(promise);
            runningPromises.splice(i, 1);
        });
        runningPromises.push(promise);

        // wait for a promise to finish if there are too many running
        if (runningPromises.length >= maxConcurrency) {
            await Promise.race(runningPromises);
        }

    });

    // wait for all promises to finish
    await Promise.all(runningPromises);

    return results

}