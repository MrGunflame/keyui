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

    public async proofTreeRoot(proof: ProofId): Promise<TreeNodeDesc> {
        return await this.send("proofTree/root", proof);
    }
}

export type ProofId = {
    env: EnvId;
    proofId: string,
};

export type EnvId = {
    envId: string,
};

export type NodeId = {
    nodeId: number;
    proofId: ProofId;
};

export type TreeNodeDesc = {
    id: NodeId;
    name: string;
};
