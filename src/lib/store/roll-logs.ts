// import { invoke } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api';
import type { ActionLog } from '../model/action-log';
import { createAsyncStore } from './asyncStore';
import type { ActionInfo } from '$lib/model/action-info.model';

export const passiveRollLogStore = createAsyncStore<ActionLog[]>([], () =>
	invoke<Array<ActionLog>>('get_action_logs')
);

export const executeAction = async (payload: ActionInfo): Promise<ActionLog> => {
	const actionLog = await invoke<ActionLog>('execute_action', { payload });
	passiveRollLogStore.update((logs) => [actionLog, ...logs]);
	return actionLog;
};
