use std::sync::Arc;

use crate::service::CompilationService;

use async_trait::async_trait;
use golem_api_grpc::proto::golem::common::{Empty, ErrorBody, ErrorsBody};
use golem_api_grpc::proto::golem::component;
use golem_api_grpc::proto::golem::componentcompilation::component_compilation_service_server::ComponentCompilationService as GrpcCompilationServer;
use golem_api_grpc::proto::golem::componentcompilation::{
    component_compilation_error, component_compilation_response, ComponentCompilationError,
    ComponentCompilationRequest, ComponentCompilationResponse,
};
use golem_common::model::ComponentId;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct CompileGrpcService {
    service: Arc<dyn CompilationService + Send + Sync>,
}

impl CompileGrpcService {
    pub fn new(service: Arc<dyn CompilationService + Send + Sync>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl GrpcCompilationServer for CompileGrpcService {
    async fn enqueue_compilation(
        &self,
        request: Request<ComponentCompilationRequest>,
    ) -> Result<tonic::Response<ComponentCompilationResponse>, Status> {
        let response = match self.enqueue_compilation_impl(request.into_inner()).await {
            Ok(_) => component_compilation_response::Result::Success(Empty {}),
            Err(e) => component_compilation_response::Result::Failure(e),
        };

        Ok(Response::new(ComponentCompilationResponse {
            result: Some(response),
        }))
    }
}

impl CompileGrpcService {
    async fn enqueue_compilation_impl(
        &self,
        request: ComponentCompilationRequest,
    ) -> Result<(), ComponentCompilationError> {
        let component_id = make_component_id(request.component_id)?;
        let component_version = request.component_version;
        self.service
            .enqueue_compilation(component_id, component_version)
            .await?;
        Ok(())
    }
}

impl From<crate::model::CompilationError> for ComponentCompilationError {
    fn from(value: crate::model::CompilationError) -> Self {
        let body = ErrorBody {
            error: value.to_string(),
        };

        let error = match value {
            crate::model::CompilationError::ComponentNotFound(_) => {
                component_compilation_error::Error::NotFound(body)
            }
            crate::model::CompilationError::CompileFailure(_)
            | crate::model::CompilationError::ComponentDownloadFailed(_)
            | crate::model::CompilationError::ComponentUploadFailed(_)
            | crate::model::CompilationError::Unexpected(_) => {
                component_compilation_error::Error::InternalError(body)
            }
        };

        ComponentCompilationError { error: Some(error) }
    }
}

fn make_component_id(
    id: Option<component::ComponentId>,
) -> Result<ComponentId, ComponentCompilationError> {
    let id = id.ok_or_else(|| bad_request_error("Missing component id"))?;
    let id: ComponentId = id
        .try_into()
        .map_err(|_| bad_request_error("Invalid component id"))?;
    Ok(id)
}

fn bad_request_error(error: impl Into<String>) -> ComponentCompilationError {
    ComponentCompilationError {
        error: Some(component_compilation_error::Error::BadRequest(ErrorsBody {
            errors: vec![error.into()],
        })),
    }
}
