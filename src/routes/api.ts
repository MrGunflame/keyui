import { invoke } from '@tauri-apps/api/core';

export class Client {
    async send(method: string, payload: any): Promise<any> {
        let resp: any = await invoke("send_msg", {
            "method": method,
            "params": payload,
        });

        return resp;
    }

    public async version(): Promise<string> {
        return await this.send("meta/version", null);
    }

    public async loadKey(content: string): Promise<ProofId> {
        return await this.send("loading/loadKey", content);
    }
}

export type ProofId = {
    env: EnvId;
    proofId: string,
};

export type EnvId = {
    envId: string,
};
