import { invoke } from '@tauri-apps/api/core';

export class Client {
    async send(method: string, payload: any): Promise<any> {
        let resp: any = await invoke("send_msg", {
            "method": method,
            "params": payload,
        });

        console.log(resp);

        if (resp.error) {
            let err = new ApiError();
            err.code = resp.error.code;
            err.data = resp.error.data;
            err.message = resp.error.message;
            err.method = method;
            throw err;
        }

        return resp.result;
    }

    public async version(): Promise<string> {
        return await this.send("meta/version", null);
    }


    public async load(params: LoadParams): Promise<ProofId> {
        let framed: any = {
            "problemFile": {
                "uri": params.problemFile,
            },
            "$class": "org.keyproject.key.api.data.LoadParams",
        };

        if (params.bootClassPath) {
            framed.bootClassPath = { "uri": params.bootClassPath };
        }

        if (params.classPath) {
            framed.classPath = params.classPath.map((x) => ({ "uri": x }));
        }

        if (params.includes) {
            framed.includes = params.includes.map((x) => ({ "uri": x }));
        }

        return await this.send("loading/load", framed);
    }

    public async loadKey(content: string): Promise<ProofId> {
        return await this.send("loading/loadKey", content);
    }

    public async proofTreeRoot(proof: ProofId): Promise<TreeNodeDesc> {
        return await this.send("proofTree/root", proof);
    }

    public async proofTreeChildren(proof: ProofId, nodeId: NodeId): Promise<TreeNodeDesc[]> {
        // FIXME: What?
        let tree_node = {
            "id": nodeId.nodeId,
            "$class": "org.keyproject.key.api.data.KeyIdentifications$TreeNodeId",
        };

        return await this.send("proofTree/children", [proof, tree_node]);
    }

    public async goalPrint(id: NodeId, options: PrintOptions): Promise<NodeTextDesc> {
        return await this.send("goal/print", [id, options]);
    }

    public async proofGoals(proof: ProofId, onlyOpened: boolean, onlyEnabled: boolean): Promise<NodeDesc> {
        return await this.send("proof/goals", [proof, onlyOpened, onlyEnabled]);
    }
}

class ApiError {
    code: number = 0;
    data: string = "";
    message: string = "";
    method: string = "";

    public toString(): string {
        return `${this.message} (Code ${this.code}) while calling ${this.method}:\n${this.data}`;
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
    nodeId: string;
    proofId: ProofId;
};

export type TreeNodeDesc = {
    id: NodeId;
    name: string;
};

export type PrintOptions = {
    unicode: boolean,
    width: number,
    indentation: number,
    pure: boolean,
    termLabels: boolean,
};

export type NodeTextDesc = {
    id: NodeTextId,
    result: string,
};

export type NodeTextId = {
    nodeId: NodeId,
    nodeTextId: number,
};

export type TreeNodeId = {
    id: string;
};

export type NodeDesc = {
    nodeId: NodeId;
    branchLabel: string;
    scriptRuleApplication: boolean;
    children: NodeDesc[];
    description: string;
};

export type LoadParams = {
    problemFile: string;
    classPath: string[] | null;
    bootClassPath: string | null;
    includes: string[] | null;
};
